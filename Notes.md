# Next

- Use lifecycle hooks for view razing.
- need preludes for bevy_quill, bevy_quill_obsidian
- Test focus.
- Think about separating styles from Quill.
  - Hard to do because text view uses the marker component.
- impl_trait_for_tuples for effect tuples.
- Recent colors in color edit.
  - Needs preferences API

## Vortex notes

- Drag to connect
- Rect select
- Keyboard
- Catalog view
  - Double-click.
- Operators
  - Buffered Nodes (for Blur)?
  - Uniform vs. Source?
    - Uniform is a node type.

Drag events:

```rust
impl<T: MyTrait + Reflect + Clone> FromType<T> for ReflectMyTrait {
  fn from_type() -> Self {
    Self {
      get_obj: |value: &dyn Reflect| {
         let value = value.downcast_ref::<T>().unwrap();
         Arc::new(RwLock::new(value.clone()))
      }
    }
  }
}
```

export type DragSource = 'node' | 'edge' | 'bg';

/\*\* Gesture lifecycle:

- - 'none' - no gesture in progress.
- - 'click' - means we clicked, but have not started dragging yet.
- - 'drag' - set when mouse moves > 3px from click point.
- - 'modal' - a modal gesture, one that ends with a click.
    \*/
    export type DragState = 'none' | 'click' | 'drag' | 'modal';

/\*\*

- A 'gesture' is a drag action or a modal state. It has several salient characteristics:
-
- 1.  There is some visual feedback which occurs when the gesture is active.
- 2.  This feedback tracks the pointer position and highlights the target objects.
- 3.  A gesture can either be completed (which causes some update of the document), or it
- can be canceled.
  \*/
  export interface IGesture {
  source: DragSource | null;

/\*_ Action to take if no dragging occurs between click and release. _/
clickAction: GestureAction | null;

/\*_ Action to take if pointer moved more than 3 pixels from start location. _/
dragAction: GestureAction | null;

/\*\*

- The node on which we which we clicked to start the drag. Can be either an entity or junctor,
- but not a group or edge.
  \*/
  anchorNode: NodeId | null;

/\*\* The edge on which we which we clicked to start the drag.

- For now, let's assume it's a node.
  \*/
  anchorEdge: EdgeId | null;

/\*_ Handle we are dragging from. _/
anchorHandle: HandleId | null;

/\*_ Initial pointer position in document coordinates. _/
anchorPos: AppPoint;

/\*_ Initial pointer position in client coordinates. _/
anchorPosClient: AppPoint;

/\*_ Node currently being hovered. Can be either an entity or junctor, but not a group or edge. _/
targetNode: NodeId | null;

/\*_ Node currently being hovered. _/
targetEdge: EdgeId | null;

/\*_ Which handle we dropped on. _/
targetHandle: HandleId | null;

/\*_ Current pointer position in document coordinates. _/
targetPos: AppPoint;

/\*_ Current pointer position in client coordinates. _/
targetPosClient: AppPoint;

/\*_ For node creation gestures, the class of node being created. _/
nodeClass?: ClassId;

/\*_ Whether this is a click, drag, or modal gesture. _/
dragState: DragState;

/\*_ Whether the mouse pointer is outside of the graph view. _/
isOut: boolean;
}
