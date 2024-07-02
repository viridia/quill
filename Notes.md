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

- Make node positions center-relative.
- Tab groups
- Catalog view
  - Catalog panel
  - Selection
  - Double-click.
- Operators
  - Attributes
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
