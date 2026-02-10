#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ElevatorError {
    CannotMoveDoorsOpen,
    InvalidFloor(u32),
    CannotOpenWhileMoving,
    DoorsAlreadyClosed,
    EmptyQueue,
}
