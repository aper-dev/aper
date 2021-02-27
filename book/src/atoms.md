# Constants and Atoms

One way to build state machines is by composing together other state machines. Aper comes with some ready-made state machines to provide common components of state management.

The simplest state machine is a `Constant`. It's a state machine whose transition has no exposed constructor,
and therefore which can never be modified once it's created. It takes an initial state, and then keeps that
state for the rest of its life.

```rust
use aper::data_structures::Constant;

fn main() {
    // Construct a constant representing an i64.
    let int_constant = Constant::new(5);
    assert_eq!(&5, int_constant.value());

    // Construct a constant representing a String.
    let string_constant = Constant::new("Hi Aper".to_string());

    assert_eq!("Hi Aper", string_constant.value().as_str());
}
```

An `Atom` is similar to a `Constant`, except that it has a transition called `ReplaceAtom`.

It represents a value that can only be changed by replacing it entirely.

```rust
use aper::data_structures::Atom;
use aper::StateMachine;

fn main() {
    let mut atom = Atom::new(5);

    // Construct a new `ReplaceAtom` transition.
    let transition = atom.replace(6);

    // Remember, calling `.replace` does not actually change any
    // state -- only a call to `.apply` can do that.
    assert_eq!(&5, atom.value());
    
    atom.apply(transition);

    // Now the change takes effect.
    assert_eq!(&6, atom.value());
}
```