use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::value::Value;

/// Errors encountered at runtime.
#[derive(Clone, Copy, Debug, Deserialize, Error, PartialEq, Serialize)]
pub enum RuntimeError {
  /// Stack overflow.
  #[error("stack overflow")]
  StackOverflow,
  /// Stack underflow.
  #[error("stack underflow")]
  StackUnderflow,
  /// Inappropriate operand.
  #[error("inappropriate operand ({1}) for instruction {0}")]
  InappropriateOperand(Instruction, Value),
  /// Inappropriate operands.
  #[error("inappropriate operands ({1}, {2}) for instruction {0}")]
  InappropriateOperands(Instruction, Value, Value),
}
