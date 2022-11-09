use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ptr::eq;

use crate::scripting_language::value::Value;
use crate::scripting_language::virtual_machine::VirtualMachine;

pub mod error;
pub use error::Error;

pub type FunctionType = fn(&VirtualMachine, &[Value]) -> Result<Value, Error>;

/// The `NativeFunction` type.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct NativeFunction(pub FunctionType);

impl Debug for NativeFunction {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let result = write!(f, "<native fn>");

    result
  }
}

impl Display for NativeFunction {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let result = write!(f, "{:#?}", self);

    result
  }
}

impl PartialEq for NativeFunction {
  fn eq(&self, other: &Self) -> bool {
    eq(self, other)
  }
}
