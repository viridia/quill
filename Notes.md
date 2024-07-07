# Next

- Use lifecycle hooks for view razing.
- need preludes for bevy_quill, bevy_quill_obsidian
- Test focus.
- Think about separating styles from Quill.
  - Hard to do because text view uses the marker component.
- impl_trait_for_tuples for effect tuples.
- Recent colors in color edit.
  - Needs preferences API
- Change window title.

## Vortex notes

- Interior controls.
- Double-click.
- Rect select
- Drag to connect
  - Start: create a proxy connection from a terminal to a point.
    - or from a point to a terminal
  - Move: update the proxy connection's endpoint, either a point or a terminal
    - query if the terminal is acceptable
  - End: remove the proxy connection and set a real one.
  - Cancel: remove the proxy connection.
- Drag to reconnect:
  - Start: hide the real connection and otherwise do the same thing.
  - End: remove the proxy connection and unhide the real one.
- Operators
  - Buffered Nodes (for Blur)?
  - Uniform vs. Source?
    - Uniform is a node type.

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

Color gradient interpolation

let v = in - origin
let d = dot(v, end - origin) / len(v) \* len(end - origin)
let d = clamp(d, 0, 1)

this.normal.dot( point ) + this.constant;

setFromNormalAndCoplanarPoint( normal, point ) {

    	this.normal.copy( normal );
    	this.constant = - point.dot( this.normal );

    	return this;

    }
