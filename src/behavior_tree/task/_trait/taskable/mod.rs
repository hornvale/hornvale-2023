use super::super::Status;
use anyhow::Error;

/// The `Taskable` trait.
pub trait Taskable {
  /// Run!
  fn run(&self) -> Result<Status, Error>;
}
