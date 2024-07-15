# Next

- Use lifecycle hooks for view razing.
- Think about separating styles from Quill.
  - Hard to do because text view uses the marker component.
- impl_trait_for_tuples for effect tuples.
- Recent colors in color edit.
  - Needs preferences API
- Change window title.
- README for top-level package should contain links to other packages.

## Vortex nextish

- Flesh out undo/redo
- Serialization
- Open/Save dialogs. (rfd)
- Make nodes do stuff!
- Commands to write:
  - validate_connection
  - propagate_changes / set modified state

## Vortex notes

- Double-click.
- Operators
  - Buffered Nodes (for Blur)?
  - Uniform vs. Source?
    - Uniform is an input node type.
- Async shaders gen:
  - set of "observed" nodes: nodes that are being used for a preview.
    - Not sure I want to store "observed" on the nodes themselves, they are really more of
      a property of the display. Buuuut....displays aren't a thing?
  - set of "dirty" nodes: nodes whose shaders are out of date.
    - this happens whenever:
      - a property is changed on the node or on an upstream node
      - an input connection is added/changed to terminal of the node or an upstream node.
      - (changes to outputs don't matter)
