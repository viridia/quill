# bevy_mod_stylebuilder

This crate provides a set of low-level utilities for configuring `bevy_ui` styles using a fluent
API. A `StyleBuilder` is an object that understands how to insert, remove, and modify Bevy style
components such as `Style`, `BackgroundColor` and so on, as well as the `Pickable` component used
by `bevy_mod_picking`.

`StyleBuilder` is extensible by implementing additional traits. In fact, all of the fluent methods
are trait methods.

```rust
use bevy_mod_stylebuilder::prelude::*;

fn style_button(ss: &mut StyleBuilder) {
    ss.border(1)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .padding((12, 0))
        .border(0)
        .color(colors::FOREGROUND)
        .cursor(CursorIcon::Pointer);
}
```

In most cases, you won't need to instantiate a `StyleBuilder` object yourself, the UI framework
will pass one to you as a callback parameter. For framework authors, however, here are the steps
needed to create a new `StyleBuilder`:

```rust
/// Construct a new StyleBuilder instance with the entity and `Styles` component.
let mut sb = StyleBuilder::new(&mut target, style);
/// Apply one or more style functions.
self.styles.apply(&mut sb);
/// Call `.finish()` to write out the changes.
sb.finish();
```

Most style components such as `BackgroundColor` are modified immediately, however `Style` is
treated as a special case because it has so many properties: it's cached in the `StyleBuilder`
instance and then flushed out at the end via `finish()`.
