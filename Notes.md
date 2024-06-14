# Next

- use_resource
- Nested components next!
- Effects w/deps
  - style_effect
  - insert effect
  - entity mutation effect
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

# Problems with view templates.

ViewTemplates, which worked really well in bevy_reactor, are having trouble here. The basic
difference is that in BR the create() function is only called once, wherease in Q create() gets
called every render. This is a problem because the return type of create is `impl View`, which
means we never explicitly declare the actual return type of the method. This in turn makes it
very difficult to store the resulting view object without using box and dyn.

The reason we need to store the view object is twofold:

- We need to store the view's state, whose type is derived from View::State.
- We need to keep around the view in order to call View.raze().

All views need to support raze, because it isn't enough to simply despawn the display nodes. When
a view component goes away, several things happen:

- the display node and its children are despawned.
- and child component owner nodes are despawned.
- any owned mutables are despawned (more generally, the tracking scope is destructed)
- any regitered cleanup handlers are run.
- possibly other stuff if the view's build is exotic.

Calling create() again, to get a View just so that we can raze it, is silly and likely inefficient.
So we need to preserve the view returned by create. But we don't know the type.

What do we want to have happen?

There are two ways in which a view template can be rebuilt:

First, the tracking scope for the VT can react. In this case, we want to re-run the create function,
and rebuild the resulting View and state. If the view's output nodes changed, we also want to notify
the invoking view (the one that called the VT) to tell it to re-attach its children.

Alternatively, the invoking view may rebuild, in which case it will construct a new instance
of the VT. If this VT is different than the previous one, we need to replace the VT, then also
re-run create() and build the view and state. Again, checking if the output nodes have changed.

So the problems we have to address are:

- comparing the previous instance of the VT with the new one (which requires the VT to derive PartialEq)
- copying the new instance of the VT over the previous one (which might mean cloning)
- copying the new instance of the View over the previous one, without knowing its type.
- copying the new view state over the old one, without knowing its type.

Note that even though we don't know the types, we do know that the old values and the new ones
have the _same_ type. That's because the type of View and State are derived from the create
method of the VT, whose type never changes.

How is all this different with presenters? (Besides the fact that we have to give up our
lovely fluent syntax):

- The type of the presenter function is knowable.
- The return type of the function is inferrable as a View.
- Props still need to be PartialEq, and possibly cloneable.
