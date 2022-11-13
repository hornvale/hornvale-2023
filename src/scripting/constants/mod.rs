use crate::scripting::error::Error;
use crate::scripting::instruction::Instruction;
use crate::scripting::value::Value;

/// The `Constants` type.
///
/// This represents a constant pool.
#[derive(Clone, Debug, Default, Display)]
#[display(fmt = "constants: {:?}", constants)]
pub struct Constants {
  pub constants: Vec<Value>,
}

impl Constants {
  /// Insert a new constant.  This returns the approprite instruction for
  /// loading the constant, which will depend on how many constants have
  /// already been added.
  pub fn push(&mut self, value: Value) -> Result<Instruction, Error> {
    let index = self.constants.len();
    self.constants.push(value);
    let result = Instruction::Constant(index as u16);
    Ok(result)
  }
}
