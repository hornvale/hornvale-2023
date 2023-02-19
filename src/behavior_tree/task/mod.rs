use super::composite::Composite;
use super::condition::Condition;
use super::decorator::Decorator;
use crate::action::Action;
use anyhow::Error;

pub mod _trait;
pub use _trait::*;
pub mod _type;
pub use _type::Status as TaskStatus;
pub use _type::*;

/// The `BehaviorTreeTask` enum.
///
/// Tasks are divided into four types:
/// - Action
/// - Condition
/// - Composite
/// - Decorator
#[derive(Clone, Debug)]
pub enum Task {
  Action(Action),
  Condition(Condition),
  Composite(Composite),
  Decorator(Decorator),
}

impl Taskable for Task {
  fn run(&self) -> Result<Status, Error> {
    Ok(Status::Success)
  }
}
