# Next

- before button:
  - How to handle children params: we need to stash the state somewhere.
  - Test callbacks.
  - Test focus.
  - Test mutable.
  - implement .effect() for elements.
- How to handle the 'disabled' property in Button - we want to access lazily so that we don't have
  to rebind event handlers.
  - create a "Disabled" component. Could be a marker.
- Asset source for obsidian
- Button
- Write tests for:
  - passing children as params (and dealing with memoization)
- Finish scope tracing.
- Effects w/deps
  - entity mutation effect
- For loops
- Component Library
- Re-enable pointer events in StyleBuilder.
- impl_trait_for_tuples for effect tuples.

# Challenges to overcome:

- dependency injection doesn't auto-track
- we don't have DeferredWorld
- we'd need react-style tracking of hooks.

# Button?

```rust
/// Button widget
#[derive(Default)]
pub struct Button {
    /// Color variant - default, primary or danger.
    pub variant: ButtonVariant,

    /// Button size.
    pub size: Size,

    /// Whether the button is disabled.
    pub disabled: bool,

    /// The content to display inside the button.
    pub children: ChildArray,

    /// Additional styles to be applied to the button.
    pub style: StyleHandle,

    /// Callback called when clicked
    pub on_click: Callback,

    /// The tab index of the button (default 0).
    pub tab_index: i32,

    /// Which corners to render rounded.
    pub corners: RoundedCorners,

    /// If true, set focus to this button when it's added to the UI.
    pub autofocus: bool,

    /// If true, render the button in a 'minimal' style with no background and reduced padding.
    pub minimal: bool,
}
```

Of these, the only "interesting" ones are the stylehandle and the callback. Both of these should
be type-erased if possible, since otherwise the resulting view signature gets difficult.

Callbacks need to be memozied (React-style), because otherwise a new callback will be generated
every time the caller updates, causing the child to unconditionally render.

Child lists will have to be wrapped in a box or arc. Unfortunately, this means that every parent
render will still unconditionally render all children, because we can't compare child lists -
not unless we make all views partialeq, which would be onerous because such as comparison would
have to walk the whole hierarchy.

OK so child lists, next problem: we can't simply box the ViewTuple like we did with views, because
we need to externalize the state. That is, we want to store the child array's state along with
the button's state, or somehoe attached to it so that they both are stored together and have the
same lifetime.

Also, we want to convert the children to a portable format early, in case we need to pass them
down multiple levels.
