# Next

- Effects w/deps
- style_effect
- Change create_entity to hook
- StyleBuilder integration
- Hooks
- Conditionals
- For loops
- Component Library
- Re-enable pointer events in StyleBuilder.

# Challenges to overcome:

- dependency injection doesn't auto-track
- we don't have DeferredWorld
- we'd need react-style tracking of hooks.

# What handles need

- entity for view component
  - has a Component containing a ViewCell
  - ViewCell contains an Arc<Mutex<dyn AnyViewState>>
  - ViewState has a concrete implementation ViewState
  - references a viewstate (view function + state)
- children
  - in bevy_reactor, children are SmallVec of ViewHandle, which is Arc
  - in quill, "children" is a separate view, which hangs on to a ViewTuple
    - I actually don't much care for this approach, since what we do with children may be
      different for different widgets.
- both bevy_reactor and quill have a problem that "views" and "view templates" are different things.
