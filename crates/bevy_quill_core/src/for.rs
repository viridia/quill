use crate::{ForIndex, View};

use super::ForEach;

/// A namespace that contains constructor functions for various kinds of for-loops:
/// * `For::each()`
/// * `For::each_cmp()`
/// * `For::index()`
pub struct For;

impl For {
    /// Construct an index for loop for an array of items. The callback is called once for each
    /// array element; its arguments are the item and the array index, and its result is a View.
    /// During rebuild, the elements are overwritten based on their current array index, so the
    /// order of child views never changes.
    pub fn index<
        Item: Send + Sync + Clone + PartialEq + 'static,
        V: View,
        F: Send + Sync + 'static + Fn(&Item, usize) -> V,
    >(
        items: &[Item],
        each_fn: F,
    ) -> ForIndex<Item, V, F, ()> {
        ForIndex::new(items, each_fn)
    }

    /// Transforms an iterator of items into an array of child views, one for each element in
    /// the original sequence. The order of child views is determined by the order of the
    /// input items. During rebuilds, the list of child views may be re-ordered based on a
    /// comparison of the generated keys.
    pub fn each_cmp<
        Item: Clone + Send + Sync,
        Iter: IntoIterator<Item = Item> + Clone + Send + Sync,
        Cmp: Fn(&Item, &Item) -> bool,
        V: View,
        F: Fn(&Item) -> V + Send,
    >(
        iter: Iter,
        cmp: Cmp,
        each: F,
    ) -> ForEach<Item, Iter, V, Cmp, F, ()> {
        ForEach::new(iter, cmp, each)
    }

    /// Transforms an iterator of items into an array of child views, one for each element in
    /// the original sequence. The order of child views is determined by the order of the
    /// input items. During rebuilds, the list of child views may be re-ordered based on a
    /// comparison of the generated keys. This version requires that the items implement
    /// `PartialEq`.
    pub fn each<
        Item: Clone + PartialEq + Send + Sync,
        Iter: IntoIterator<Item = Item> + Clone + Send + Sync,
        V: View,
        F: Fn(&Item) -> V + Send,
    >(
        iter: Iter,
        each: F,
    ) -> ForEach<Item, Iter, V, impl Fn(&Item, &Item) -> bool, F, ()> {
        ForEach::new(iter, |a, b| a == b, each)
    }
}
