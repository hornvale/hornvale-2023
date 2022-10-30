use crate::scripting_language::virtual_machine::error::Error as VirtualMachineError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Error)]
pub enum Error {
  /// A general error occurred.
  #[error("an error occurred ({0})")]
  GeneralError(#[from] Box<VirtualMachineError>),
}
