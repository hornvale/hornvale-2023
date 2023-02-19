use anyhow::Error;

/// The `BehaviorTreeTaskStatus` enum.
#[derive(Debug)]
pub enum Status {
  Success,
  Failure(String),
  Running(usize),
  Error(Error),
}
