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

- Delete selected nodes.

  - Crash when deleting. This happens (I think) because the view which displays the node
    updates sooner than the view which iterates over the nodes; which means that the node view
    is trying to display an entity that has been deleted.

    When I delete a node, components get updated in this order:

    - GraphNodeView
    - GraphNodeView
    - GraphView
    - GraphNodePropertyView
    - GraphNodePropertyView

- Dragging of edges.
- Flesh out undo/redo
- Serialization
- Open/Save dialogs. (rfd)

## Vortex notes

- Double-click.
- Rect select
- Drag to reconnect:
  - Start: hide the real connection and otherwise do the same thing.
  - End: remove the proxy connection and unhide the real one.
- Operators
  - Buffered Nodes (for Blur)?
  - Uniform vs. Source?
    - Uniform is an input node type.
