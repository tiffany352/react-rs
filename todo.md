### Reconciler

- [x] `VirtualTree::mount`
- [x] `VirtualTree::render`
- [ ] `VirtualTree::update`
- [ ] `VirtualTree::unmount`
- [ ] Element keys
- [ ] Fragments
- [ ] Context

### Lifecycle methods

- [x] `create` (`constructor` + `init`)
- [x] `did_mount`
- [x] `render`
- [ ] `should_update`
- [ ] `get_derived_state_from_props`
- [ ] `will_update`
- [ ] `did_update`
- [ ] `will_unmount`

### Snax macro

Use the snax crate to implement a JSX-like proc macro for creating
elements.

### Portals & Refs

The intention of this crate is to be directly integrated with a GUI
framework, instead of being a layer over top of a DOM-oriented one like
the web. As a result, portals and refs aren't really useful, and have
confusing semantics.

### State

Experiment with ways of expressing `setState` and related patterns in
the constraints of Rust's type system.

Some ideas here:

- Some kind of message passing token, with a third associated type on
  the component for which types of messages it receives. There would be
  a lifecycle method something like this:
  
  ```rs
  fn on_message(
      &mut self,
      _message: Self::Message,
      old_state: Self::State
  ) -> Self::State {
      // Default implementation.
      old_state
  }
  ```
- Need to figure how how passing callbacks to child components as props
  will work. Lifetimes get messy and any closures can't close over the
  component itself.

### Context

The older style `_context` API is probably not workable in this case.
It's likely a better plan to use a Provider/Accessor pattern, like
React's new context API.
