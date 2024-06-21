# Next

- before button:
  - How to handle children params: we need to stash the state somewhere.
  - Test callbacks.
  - Test focus.
  - Test mutable.
  - Test disabled
  - Dispose mutables / TrackingScope cleanup.
- Callbacks should be one-shot systems!
- TrackingScope::raze never gets called on the root.
- How to handle the 'disabled' property in Button - we want to access lazily so that we don't have
  to rebind event handlers.
  - create a "Disabled" component. Could be a marker.
- Finish scope tracing.
- Effects w/deps
  - general entity mutation effect
- For loops
- Component Library
- impl_trait_for_tuples for effect tuples.
