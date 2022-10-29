use std::num::ParseFloatError;

use crate::scripting_language::compiler::error::Error as CompilerError;
use crate::scripting_language::error::Error as ScriptingLanguageError;
use crate::scripting_language::scanner::error::Error as ScannerError;
use crate::scripting_language::token::r#type::Type as TokenType;

/// Errors encountered during the parsing process.
#[derive(Clone, Debug, Error)]
pub enum Error {
  /// Unknown error.
  #[error("an unknown error occurred")]
  UnknownError,
  /// Unexpected token.
  #[error("unexpected token {0} ({1})")]
  UnexpectedTokenError(TokenType, String),
  /// Multiple errors occurred.
  #[error("multiple errors occurred ({0:#?})")]
  MultipleErrors(Vec<String>),
  /// Scanner error.
  #[error("an error occurred in the scanner ({0})")]
  ScannerError(#[from] ScannerError),
  /// ParseFloat error.
  #[error("an error occurred parsing a float ({0})")]
  ParseFloatError(#[from] ParseFloatError),
  /// General error.
  #[error("an error occurred in the scripting language ({0})")]
  ScriptingLanguageError(#[from] ScriptingLanguageError),
  /// Expected an expression.
  #[error("expected an expression")]
  ExpectedExpression,
  /// Invalid assignment target.
  #[error("invalid assignment target")]
  InvalidAssignmentTarget,
  /// Attempted to declare a variable with the same name as an existing
  /// variable in the same scope.
  #[error("attempted to declare a variable with the same name as an existing variable in the same scope")]
  AttemptedToRedeclareVariable,
  /// Attempted to create a function with way too damned many parameters.
  #[error("attempted to declare a function with too many parameters")]
  FunctionArityExceededLimit,
  /// Attempted to call a function with way too damned many arguments.
  #[error("attempted to call with too many arguments")]
  FunctionCallArgumentsExceededLimit,
  /// Tried to exit into the surrounding context of an unparented compiler.
  #[error("attempted to pop a compiler without an enclosing copiler")]
  TriedToPopTopCompiler,
  /// Attempted to use `this` outside of a class context.
  #[error("attempted to use `this` outside of a class context.")]
  AttemptedToUseThisOutsideClass,
  /// Attempted to subclass itself.
  #[error("attempted to make a class subclass itself.")]
  AttemptedToDeclareClassAsASubclassOfItself,
  /// Compiler error.
  #[error("an error occurred in the compiler ({0})")]
  CompilerError(#[from] CompilerError),
}
