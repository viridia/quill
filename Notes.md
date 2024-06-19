# Next

- Mutable
- Asset source
- Button
- Write tests for:
  - passing children as params (and dealing with memoization)
- Finish scope tracing.
- Effects w/deps
  - style_effect
  - insert effect
  - entity mutation effect
- Change create_entity to hook
  - Need for scope to track hook order.
- Hooks
- For loops
- Component Library
- Re-enable pointer events in StyleBuilder.

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

# Problems with attach_children:

The problem is what happens when you have the following hierarchy:

- View Templates A, B and C where B is a child of A and C is a child of B.
- C changes its output
- B is a simple view that merely returns output of C.
- In this case, A needs to be notified so that it can re-attach its children.

The boolean result from C.rebuild() doesn't help here, because C is being changed
independently from B.
