# Next

- Test focus.
- Finish controls:
  - Menu
  - Disclosure
- Think about separating styles from Quill.
  - Hard to do because text view uses the marker component.
- Experiment with observer-based destructors.
  - Start with TrackingScope::raze().
  - Need to migrate to DeferredWorld.
- TrackingScope::raze never gets called on the root.
- Finish scope tracing.
- impl_trait_for_tuples for effect tuples.
- Document:
  - callbacks
  - other hooks
  - portals
- Recent colors in color edit.
- Optimize swatch grid with memo.
