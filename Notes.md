# Next

- Use lifecycle hooks for view razing.
- Test focus.
- Think about separating styles from Quill.
  - Hard to do because text view uses the marker component.
- impl_trait_for_tuples for effect tuples.
- Recent colors in color edit.
  - Needs preferences API
- Change window title.

## Vortex nextish

- Color popup menu not appearing.
- Fix problems with drag state machine.
- Test auto-deletion of connections.
- Flesh out undo/redo
- Serialization

## Vortex notes

- Double-click.
- Rect select
- Drag to reconnect:
  - Start: hide the real connection and otherwise do the same thing.
  - End: remove the proxy connection and unhide the real one.
- Operators
  - Buffered Nodes (for Blur)?
  - Uniform vs. Source?
    - Uniform is a node type.
