use super::task::{TaskStatus, Taskable};
use anyhow::Error;

/// The `BehaviorTreeDecorator` type.
#[derive(Clone, Debug)]
pub struct Decorator {}

impl Taskable for Decorator {
  fn run(&self) -> Result<TaskStatus, Error> {
    Ok(TaskStatus::Success)
  }
}
