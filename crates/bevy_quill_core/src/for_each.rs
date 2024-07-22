use std::ops::Range;

use bevy::ecs::world::{DeferredWorld, World};

use crate::{lcs::lcs, node_span::NodeSpan, Cx, View};

pub struct ListItem<Value: Clone, V: View> {
    value: Value,
    view: Option<V>,
    state: Option<V::State>,
}

impl<Value: Clone, V: View> ListItem<Value, V> {
    fn nodes(&self, world: &World) -> NodeSpan {
        self.view
            .as_ref()
            .unwrap()
            .nodes(world, self.state.as_ref().unwrap())
    }

    fn raze(&mut self, world: &mut DeferredWorld) {
        if let (Some(ref view), Some(mut state)) = (self.view.take(), self.state.take()) {
            view.raze(world, &mut state);
        }
    }
}

#[doc(hidden)]
pub struct ForEach<
    Item: Send + Clone,
    Iter: IntoIterator<Item = Item> + Clone,
    V: View,
    Cmp: Fn(&Item, &Item) -> bool,
    F: Fn(&Item) -> V + Send,
    FB: View,
> {
    iter: Iter,
    cmp: Cmp,
    each: F,
    fallback: Option<FB>,
}

impl<
        Item: Send + Clone,
        Iter: IntoIterator<Item = Item> + Clone,
        V: View,
        Cmp: Fn(&Item, &Item) -> bool,
        F: Fn(&Item) -> V + Send,
    > ForEach<Item, Iter, V, Cmp, F, ()>
{
    pub fn new(iter: Iter, cmp: Cmp, each: F) -> Self {
        Self {
            iter,
            cmp,
            each,
            fallback: None,
        }
    }
}

impl<
        Item: Send + Clone,
        Iter: IntoIterator<Item = Item> + Clone,
        V: View,
        Cmp: Fn(&Item, &Item) -> bool,
        F: Fn(&Item) -> V + Send,
        FB: View,
    > ForEach<Item, Iter, V, Cmp, F, FB>
where
    V::State: Clone,
{
    pub fn with_fallback<FB2: View>(self, fallback: FB2) -> ForEach<Item, Iter, V, Cmp, F, FB2> {
        ForEach::<Item, Iter, V, Cmp, F, FB2> {
            iter: self.iter,
            each: self.each,
            cmp: self.cmp,
            fallback: Some(fallback),
        }
    }

    /// Uses the sequence of key values to match the previous array items with the updated
    /// array items. Matching items are patched, other items are inserted or deleted.
    ///
    /// # Arguments
    ///
    /// * `cx` - Context used to build individual elements.
    /// * `prev_state` - Array of view state elements from previous update.
    /// * `prev_range` - The range of elements we are comparing in `prev_state`.
    /// * `next_state` - Array of view state elements to be built.
    /// * `next_range` - The range of elements we are comparing in `next_state`.
    /// * `out` - Array to store the new view state elements.
    #[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
    fn build_recursive(
        &self,
        cx: &mut Cx,
        prev_state: &mut [ListItem<Item, V>],
        prev_range: Range<usize>,
        next_items: &[Item],
        next_range: Range<usize>,
        out: &mut Vec<ListItem<Item, V>>,
    ) -> bool {
        let mut changed = false;

        // Look for longest common subsequence.
        // prev_start and next_start are *relative to the slice*.
        let (prev_start, next_start, lcs_length) = lcs(
            &prev_state[prev_range.clone()],
            &next_items[next_range.clone()],
            |a, b| (self.cmp)(&a.value, b),
        );

        // If there was nothing in common
        if lcs_length == 0 {
            // Raze old elements
            for i in prev_range {
                prev_state[i].raze(&mut DeferredWorld::from(cx.world_mut()));
                changed = true;
            }
            // Build new elements
            for i in next_range {
                changed = true;
                let view = (self.each)(&next_items[i]);
                let state = view.build(cx);
                out.push(ListItem {
                    value: next_items[i].clone(),
                    view: Some(view),
                    state: Some(state),
                });
            }
            return changed;
        }

        // Adjust prev_start and next_start to be relative to the entire state array.
        let prev_start = prev_start + prev_range.start;
        let next_start = next_start + next_range.start;

        // Stuff that precedes the LCS.
        if prev_start > prev_range.start {
            if next_start > next_range.start {
                // Both prev and next have entries before lcs, so recurse
                changed |= self.build_recursive(
                    cx,
                    // view_entity,
                    prev_state,
                    prev_range.start..prev_start,
                    next_items,
                    next_range.start..next_start,
                    out,
                )
            } else {
                // Deletions
                for i in prev_range.start..prev_start {
                    prev_state[i].raze(&mut DeferredWorld::from(cx.world_mut()));
                    changed = true;
                }
            }
        } else if next_start > next_range.start {
            // Insertions
            for i in next_range.start..next_start {
                let view = (self.each)(&next_items[i]);
                let state = view.build(cx);
                out.push(ListItem {
                    value: next_items[i].clone(),
                    view: Some(view),
                    state: Some(state),
                });
                changed = true;
            }
        }

        // For items that match, copy over the view and value.
        for i in 0..lcs_length {
            let prev = &mut prev_state[prev_start + i];
            out.push(ListItem {
                value: prev.value.clone(),
                view: prev.view.take(),
                state: prev.state.take(),
            });
        }

        // Stuff that follows the LCS.
        let prev_end = prev_start + lcs_length;
        let next_end = next_start + lcs_length;
        if prev_end < prev_range.end {
            if next_end < next_range.end {
                // Both prev and next have entries after lcs, so recurse
                changed |= self.build_recursive(
                    cx,
                    // view_entity,
                    prev_state,
                    prev_end..prev_range.end,
                    next_items,
                    next_end..next_range.end,
                    out,
                );
            } else {
                // Deletions
                for i in prev_end..prev_range.end {
                    prev_state[i].raze(&mut DeferredWorld::from(cx.world_mut()));
                    changed = true;
                }
            }
        } else if next_end < next_range.end {
            // Insertions
            for i in next_end..next_range.end {
                let view = (self.each)(&next_items[i]);
                let state = view.build(cx);
                out.push(ListItem {
                    value: next_items[i].clone(),
                    view: Some(view),
                    state: Some(state),
                });
                changed = true;
            }
        }

        changed
    }
}

impl<
        Item: Send + Sync + Clone + 'static,
        Iter: IntoIterator<Item = Item> + Clone + Send + Sync + 'static,
        V: View,
        Cmp: Fn(&Item, &Item) -> bool + Send + Sync + 'static,
        F: Fn(&Item) -> V + Send + Sync + 'static,
        FB: View,
    > View for ForEach<Item, Iter, V, Cmp, F, FB>
where
    V::State: Clone,
{
    type State = (Vec<ListItem<Item, V>>, Option<FB::State>);

    fn nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        let mut child_spans: Vec<NodeSpan> = state.0.iter().map(|item| item.nodes(world)).collect();
        if let Some(ref fallback) = self.fallback {
            if let Some(ref fbstate) = state.1 {
                child_spans.push(fallback.nodes(world, fbstate));
            }
        }
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn build(&self, cx: &mut Cx) -> Self::State {
        let mut state = (Vec::new(), None);
        self.rebuild(cx, &mut state);
        state
    }

    fn rebuild(&self, cx: &mut Cx, state: &mut Self::State) -> bool {
        let items = self.iter.clone().into_iter().collect::<Vec<_>>();
        let next_len = items.len();
        let mut next_state: Vec<ListItem<Item, V>> = Vec::with_capacity(next_len);
        let prev_len = state.0.len();

        let mut changed = self.build_recursive(
            cx,
            &mut state.0,
            0..prev_len,
            &items,
            0..next_len,
            &mut next_state,
        );

        // Handle fallback
        if let Some(ref fallback) = self.fallback {
            match state.1 {
                // If there are > 0 items, destroy fallback if present.
                Some(ref mut fb_ent) if next_len > 0 => {
                    fallback.raze(&mut DeferredWorld::from(cx.world_mut()), fb_ent);
                    state.1 = None;
                    changed = true;
                }

                // If there are no items, render fallback unless already rendered.
                None if next_len == 0 => {
                    state.1 = Some(fallback.build(cx));
                    changed = true;
                }

                // Otherwise, no change.
                _ => {}
            }
        }

        #[allow(clippy::needless_range_loop)]
        for j in 0..next_len {
            assert!(next_state[j].state.is_some(), "Empty state: {}", j);
        }
        std::mem::swap(&mut state.0, &mut next_state);
        changed
    }

    fn attach_children(&self, world: &mut World, state: &mut Self::State) -> bool {
        let mut changed = false;
        for child_state in state.0.iter_mut() {
            if let Some(ref view) = child_state.view {
                changed |= view.attach_children(world, child_state.state.as_mut().unwrap());
            }
        }
        if let Some(ref mut fbstate) = state.1 {
            changed |= self
                .fallback
                .as_ref()
                .unwrap()
                .attach_children(world, fbstate);
        }
        changed
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        for child_state in state.0.iter_mut() {
            if let Some(ref view) = child_state.view {
                view.raze(world, child_state.state.as_mut().unwrap());
            }
        }
        if let Some(ref mut fbstate) = state.1 {
            self.fallback.as_ref().unwrap().raze(world, fbstate);
        }
    }
}
