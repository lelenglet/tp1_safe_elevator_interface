#[derive(Debug, PartialEq, Eq, Clone,Copy)]
pub enum State {
    MovingUp,
    MovingDown,
    DoorsOpen,
    Idle,
}
