use bevy::ecs::world::{DeferredWorld, World};

use crate::View;

use crate::node_span::NodeSpan;

pub struct IndexedListItem<V: View> {
    view: Option<V>,
    state: V::State,
}

impl<V: View> IndexedListItem<V> {
    fn nodes(&self, world: &World) -> NodeSpan {
        self.view.as_ref().unwrap().nodes(world, &self.state)
    }
}

#[doc(hidden)]
pub struct ForIndex<Item: Send + Clone, V: View, F: Fn(&Item, usize) -> V + Send, FB: View> {
    items: Vec<Item>,
    each: F,
    fallback: Option<FB>,
}

impl<Item: Send + Clone, V: View, F: Fn(&Item, usize) -> V + Send> ForIndex<Item, V, F, ()> {
    pub fn new(items: &[Item], each: F) -> Self {
        Self {
            items: Vec::from(items),
            each,
            fallback: None,
        }
    }
}

impl<Item: Send + Clone, V: View, F: Fn(&Item, usize) -> V + Send, FB: View>
    ForIndex<Item, V, F, FB>
{
    pub fn with_fallback<FB2: View>(self, fallback: FB2) -> ForIndex<Item, V, F, FB2> {
        ForIndex::<Item, V, F, FB2> {
            items: self.items,
            each: self.each,
            fallback: Some(fallback),
        }
    }
}

impl<
        Item: Send + Sync + Clone + 'static,
        V: View,
        F: Fn(&Item, usize) -> V + Send + Sync + Clone + 'static,
        FB: View,
    > View for ForIndex<Item, V, F, FB>
where
    V::State: Clone,
{
    type State = (Vec<IndexedListItem<V>>, Option<FB::State>);

    fn nodes(&self, world: &World, state: &Self::State) -> NodeSpan {
        let mut child_spans: Vec<NodeSpan> = state.0.iter().map(|item| item.nodes(world)).collect();
        if let Some(ref fallback) = self.fallback {
            if let Some(ref fbstate) = state.1 {
                child_spans.push(fallback.nodes(world, fbstate));
            }
        }
        NodeSpan::Fragment(child_spans.into_boxed_slice())
    }

    fn build(&self, cx: &mut crate::Cx) -> Self::State {
        let mut state = (Vec::new(), None);
        self.rebuild(cx, &mut state);
        state
    }

    fn rebuild(&self, cx: &mut crate::Cx, state: &mut Self::State) -> bool {
        let next_len = self.items.len();
        let mut prev_len = state.0.len();
        // let mut child_spans: Vec<NodeSpan> = Vec::with_capacity(next_len);
        // child_spans.resize(next_len, NodeSpan::Empty);

        // Overwrite existing items.
        // TODO: Blind overwriting might be a problem here if, for example, we overwrite
        // a text element with a non-text element. Basically we're not razing the old output
        // (because we don't know if we should) and this could cause leftovers. If only views
        // were comparable!
        let mut i = 0usize;
        let mut changed = false;
        while i < next_len && i < prev_len {
            let child_state = &mut state.0[i];
            child_state.view = Some((self.each)(&self.items[i], i));
            changed |= child_state
                .view
                .as_ref()
                .unwrap()
                .rebuild(cx, &mut child_state.state);
            // child_spans[i] = child_state.node.clone();
            i += 1;
        }

        // Append new items
        while i < next_len {
            let view = (self.each)(&self.items[i], i);
            let st = view.build(cx);
            state.0.push(IndexedListItem {
                view: Some(view),
                state: st,
            });
            i += 1;
            changed = true;
        }

        // Raze surplus items.
        while i < prev_len {
            prev_len -= 1;
            let child_state = &mut state.0[prev_len];
            if let Some(ref view) = child_state.view {
                view.raze(
                    &mut DeferredWorld::from(cx.world_mut()),
                    &mut child_state.state,
                );
            }
            state.0.pop();
            changed = true;
        }

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

        changed
    }

    fn raze(&self, world: &mut DeferredWorld, state: &mut Self::State) {
        let prev_len = state.0.len();

        let mut i = 0usize;
        while i < prev_len {
            let child_state = &mut state.0[i];
            if let Some(ref view) = child_state.view {
                view.raze(world, &mut child_state.state);
            }
            i += 1;
        }
        if let Some(ref mut fbstate) = state.1 {
            self.fallback.as_ref().unwrap().raze(world, fbstate);
        }
    }
}
