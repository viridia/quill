# bevy_quill_obsidian

**Obsidian** is an opinionated widget library written for Bevy+Quill. The widgets are intended
for use in constructing editors, and their color scheme and visual design is based on the
mocks found here: https://amytimed.github.io/bevy_editor_mockup/

## Controls

- `Button` and `IconButton`
- `Checkbox`
- `ColorEdit`
- `Dialog`
- `DisclosureToggle`
- `Slider` and `GradientSlider`
- `Spin`
- `ListView`
- `MenuButton` and `MenuPopup`
- `ScrollView`
- `Swatch` and `SwatchGrid`
- `Splitter`
- `ToolPalette` and `ToolButton`

## Hooks

- `create_bistable_transition` - timer-based transitions for widgets that have an enter/exit animation.
- `use_element_rect` - returns the logical screen bounds of a UI entity.
- `is_focused`, `is_focus_within`, `is_focus_visible`, `is_focus_within_visible` - hooks that return true if the
  element (or one of it's children) have keyboard focus.
- `is_hovered` - hook that returns true if the pointer is hovering over the entity.

## Animations

Animation components that can be inserted into a UI node:

- `AnimatedBackgroundColor`
- `AnimatedBorderColor`
- `AnimatedPxWidth`
- `AnimatedPxHeight`
- `AnimatedScale`
- `AnimatedRotation`
- `AnimatedTranslation`

## Focus handling and tab navigation

Obsidian implements tab navigation using the `bevy_a11y` `Focus` resource.

- `TabIndex` - insert this into a UI node to indicate that it can have keyboard focus.
- `TabGroup` - insert this at the root of your UI hierarchy to enable tab navigation. You can also
  have additional "modal" tab groups to allow "tab trapping".
- `KeyCharEvent` - event which is dispatched to the current focus element (using `bevy_eventlistener`) when a character is typed.
- `KeyPressEvent` - event which is dispatched to the current focus element (using `bevy_eventlistener`) when a key is pressed.
- `DefaultKeyListener` - entity that receives keyboard events when no element has focus.

In general, you should avoid using the built-in Bevy keyboard events for things like global shortcuts.
The reason is because this doesn't allow the current focus widget to cancel the key event. For
example, if you hit `backspace` while editing a string in a text input widget, you probably want
it to just delete a character, and not also delete whatever object you had currently selected.

Keyboard events are always dispatched to the current focused element and bubble upward from there.
If no element has focus, then the key event is dispatched to the first entity that has a
`DefaultKeyListener` component.

## Standard styles and colors

- `colors` - the standard editor color theme.
- `typography` - the standard editor text styles (currently using OpenSans, which is included).
