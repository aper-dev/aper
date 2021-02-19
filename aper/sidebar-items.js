initSidebarItems({"enum":[["StateUpdateMessage","A message from the server to a client that tells it to update its state."]],"mod":[["data_structures",""]],"struct":[["PlayerID","An opaque identifier for a single connected user."],["SuspendedEvent","Represents a transition that a [crate::StateMachine] would like to receive in the future from the server, as well as the time that it would like to receive it. Each state machine can have either zero or one transition suspended at any point in time, and may change or remove that suspended event every time its state is updated (and only then)."],["TransitionEvent","A transition with associated metadata: which player triggered it and when. The player ID is optional, since `SuspendedEvent`s do not have a player associated with them."]],"trait":[["StateMachine","This trait provides the methods that Aper needs to be able to interact with an object as a state machine."],["StateMachineFactory","A trait indicating that a struct can be used to create a [StateMachine] for a given type. If your [StateMachine] does not need to be initialized with any external data or state, implement [std::default::Default] on it to avoid the need for a factory."]]});