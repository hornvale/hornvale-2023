use super::task::{TaskStatus, Taskable};
use anyhow::Error;

/// The `BehaviorTreeComposite` type.
#[derive(Clone, Debug)]
pub struct Composite;

impl Taskable for Composite {
  fn run(&self) -> Result<TaskStatus, Error> {
    Ok(TaskStatus::Success)
  }
}
