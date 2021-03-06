use crate::{StateMachine, Transition, TransitionEvent};
use serde::{Deserialize, Serialize};

/// This trait can be added to a [StateMachine] which takes a [TransitionEvent] as
/// its transition. Only state machines with this trait can be used directly with
/// the aper client/server infrastructure.
pub trait StateProgram<T: Transition>: StateMachine<Transition = TransitionEvent<T>> {
    /// A state machine may "suspend" an event which occurs at a specific time in the future.
    /// This is useful for ensuring that the state is updated at a future time regardless of
    /// a user-initiated state change before then. State machines that only change state as a
    /// result of user-initiated events can ignore this method, as the default implementation
    /// is to never suspend an event.
    ///
    /// This method is called by the server once after every call to `process_event`. If it
    /// returns `None`, no event is suspended, and any previously suspended event is canceled.
    /// If it returns `Some`, the provided event becomes the "suspended" event, replacing the
    /// prior suspended event if there was one.
    ///
    /// Only one event can be suspended at a time. If a state machine wants to be triggered for
    /// multiple events in the future, it is up to that state machine to return the
    /// (chronologically) next event each time this method is called.
    ///
    /// Currently, only the state machine running on the server ever has this method called.
    ///
    /// Since they are not associated with a particular player, suspended events trigger
    /// `process_event` with a `None` as the player in the [TransitionEvent].
    fn suspended_event(&self) -> Option<TransitionEvent<T>> {
        None
    }
}

/// A trait indicating that a struct can be used to create a [StateProgram] for a given type.
pub trait StateProgramFactory<T: Transition, State: StateProgram<T>>:
    Sized + Unpin + 'static + Send
{
    fn create(&mut self) -> State;
}

/// A [StateProgram] implementation that can be built from any [StateMachine]. Transitions
/// are stripped of their metadata and passed down to the underlying state machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(bound = "")]
pub struct StateMachineContainerProgram<SM: StateMachine>(pub SM);

impl<SM: StateMachine> StateMachine for StateMachineContainerProgram<SM> {
    type Transition = TransitionEvent<SM::Transition>;

    fn apply(&mut self, transition: Self::Transition) {
        self.0.apply(transition.transition);
    }
}

impl<SM: StateMachine> StateProgram<SM::Transition> for StateMachineContainerProgram<SM> {}
