use crate::scripting::instruction::Instruction;
use crate::scripting::value::Value;

/// Errors encountered at runtime.
#[derive(Clone, Debug, Error)]
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
  /// Undefined variable.
  #[error("encountered a reference to an undefined variable '{0}'")]
  UndefinedVariable(String),
  /// Attempted to call something that wasn't a function.
  #[error("attempted to call a non-callable value '{0}'")]
  CalledUncallableValue(Value),
  /// Called a function with an unexpected number of arguments.
  #[error("attempted to call a function with {0} arguments (expected {1})")]
  CalledFunctionWithWrongNumberOfArguments(usize, usize),
  /// Class initializer was called, but it is not a closure.
  #[error("attempted to initialize a class instance with something other than a closure")]
  ClassInitializerWasNotAClosure,
  /// Class initializer was called with the wrong number of arguments.
  #[error("attempted to initialize a class instance with {0} arguments (expected 0)")]
  ClassInitializerCalledWithArguments(usize),
  /// Tried to define a method outside of a class context.
  #[error("attempted to define method outside class context")]
  DefinedMethodOutsideClassContext,
  /// Tried to call a method on something other than an instance.
  #[error("attempted to call method on non-instance")]
  CalledMethodOnNonInstance,
  /// Tried to call a method that is not a closure.
  #[error("attempted to call non-closure method")]
  CalledNonClosureMethod,
  /// Tried to call a method that does not exist.
  #[error("attempted to call non-existent method")]
  CalledNonexistentMethod,
  /// Tried to access a property on a non-instance.
  #[error("attempted to access property on non-instance")]
  AccessedPropertyOnNonInstance,
  /// Tried to subclass something that wasn't a class.
  #[error("attempted to subclass something that wasn't a class")]
  AttemptedToSubclassNonClass,
  /// Called `super` but couldn't find a superclass.
  #[error("could not find superclass")]
  CouldNotFindRequestedSuperclass,
  /// Tried to access an undefined property.
  #[error("attempted to access undefined proeprty '{0}'")]
  UndefinedProperty(String),
}
