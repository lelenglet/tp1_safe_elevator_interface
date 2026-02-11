use crate::{ElevatorError, State};
use std::collections::{VecDeque, vec_deque};

#[derive(Clone)]
pub struct Elevator {
    pub floor: u32,
    pub state: super::state::State,
    pub queue: vec_deque::VecDeque<u32>,
}

impl Elevator {
    pub fn new(f: u32) -> Result<Self, std::io::Error> {
        if f > 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "floor is greater than 5",
            ));
        }
        Ok(Elevator {
            floor: f,
            state: State::Idle,
            queue: vec_deque::VecDeque::new(),
        })
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn floor(&self) -> u32 {
        self.floor
    }

    pub fn queue(&self) -> VecDeque<u32> {
        self.queue.clone()
    }

    pub fn call(&mut self, floor: u32) -> Result<(), ElevatorError> {
        if floor > 5 {
            return Err(ElevatorError::InvalidFloor(floor));
        }
        if self.floor != floor && !self.queue.contains(&floor) {
            if floor < self.floor {
                self.state = State::MovingDown;
            } else if floor > self.floor {
                self.state = State::MovingUp;
            }
            self.queue.push_back(floor);
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
        if self.queue().is_empty() {
            return Err(ElevatorError::EmptyQueue);
        }
        if self.state == State::MovingUp {
            self.floor += 1;
        } else if self.state == State::MovingDown {
            self.floor -= 1;
        }
        if &self.floor == self.queue.front().unwrap() {
            self.state = State::DoorsOpen;
            self.queue.pop_front().unwrap();
        }
        Ok(())
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen && self.state != State::Idle {
            return Err(ElevatorError::CannotOpenWhileMoving);
        }
        self.state = State::DoorsOpen;
        Ok(())
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::Idle {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
        } else {
            self.call(*self.queue.front().unwrap())?;
        }
        Ok(())
    }

    pub fn status(&self) -> Self {
        self.clone()
    }
}
