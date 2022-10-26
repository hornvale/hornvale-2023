use crate::scripting_language::error::Error;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::value::Value;

/// The `Constants` type.
///
/// This represents a constant pool.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Constants {
  pub constants: Vec<Value>,
}

impl Constants {
  /// Insert a new constant.  This returns the approprite instruction for
  /// loading the constant, which will depend on how many constants have
  /// already been added.
  #[named]
  pub fn push(&mut self, value: Value) -> Result<Instruction, Error> {
    trace_enter!();
    trace_var!(value);
    let index = self.constants.len();
    self.constants.push(value);
    // Use an appropriate instruction for the size of the constant index.
    let result = match index {
      index if index <= u8::MAX.into() => Instruction::Constant(index as u8),
      index => Instruction::LongConstant(index as u16),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
