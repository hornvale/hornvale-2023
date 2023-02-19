use super::task::{TaskStatus, Taskable};
use anyhow::Error;

/// The `BehaviorTreeCondition` type.
#[derive(Clone, Debug)]
pub struct Condition {}

impl Taskable for Condition {
  fn run(&self) -> Result<TaskStatus, Error> {
    Ok(TaskStatus::Success)
  }
}
