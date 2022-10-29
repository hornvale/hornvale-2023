use std::num::ParseFloatError;

use crate::scripting_language::compiler::error::Error as CompilerError;
use crate::scripting_language::error::Error as ScriptingLanguageError;
use crate::scripting_language::scanner::error::Error as ScannerError;
use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;

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
  #[error("an error occurred ({0})")]
  ScriptingLanguageError(#[from] ScriptingLanguageError),
  /// Expected an expression.
  #[error("expected an expression at {0:#?}")]
  ExpectedExpression(Option<Token>),
  /// Invalid assignment target.
  #[error("invalid assignment target {0:#?}")]
  InvalidAssignmentTarget(Option<Token>),
  /// Attempted to declare a variable with the same name as an existing
  /// variable in the same scope.
  #[error("attempted to declare a variable with the same name as an existing variable in the same scope {0:#?}")]
  AttemptedToRedeclareVariable(Option<Token>),
  /// Attempted to create a function with way too damned many parameters.
  #[error("attempted to declare a function with too many parameters {0:#?}")]
  FunctionArityExceededLimit(Option<Token>),
  /// Attempted to call a function with way too damned many arguments.
  #[error("attempted to call with too many arguments {0:#?}")]
  FunctionCallArgumentsExceededLimit(Option<Token>),
  /// Tried to exit into the surrounding context of an unparented compiler.
  #[error("attempted to pop a compiler without an enclosing copiler {0:#?}")]
  TriedToPopTopCompiler(Option<Token>),
  /// Attempted to use `this` outside of a class context.
  #[error("attempted to use `this` outside of a class context.")]
  AttemptedToUseThisOutsideClass(Option<Token>),
  /// Attempted to subclass itself.
  #[error("attempted to make a class subclass itself.")]
  AttemptedToDeclareClassAsASubclassOfItself,
  /// Compiler error.
  #[error("an error occurred in the compiler ({0})")]
  CompilerError(#[from] CompilerError),
}
