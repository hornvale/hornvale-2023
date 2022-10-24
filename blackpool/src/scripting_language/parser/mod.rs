use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::program::Program;
use crate::scripting_language::scanner::Scanner;
use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;

pub mod error;
use error::Error;

/// The `Parser` type.
#[derive(Clone, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[display(fmt = "current: {:#?}, previous: {:#?}", current, previous)]
pub struct Parser {
  /// The current token.
  pub current: Option<Token>,
  /// The last token processed.
  pub previous: Option<Token>,
  /// Whether we should suppress new errors ("Panic Mode").
  pub suppress_new_errors: bool,
  /// Whether we have actually encountered an error.
  pub did_encounter_error: Option<Error>,
}

impl Parser {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let current = None;
    trace_var!(current);
    let previous = None;
    trace_var!(previous);
    let suppress_new_errors = false;
    trace_var!(suppress_new_errors);
    let did_encounter_error = None;
    trace_var!(did_encounter_error);
    let result = Self {
      current,
      previous,
      suppress_new_errors,
      did_encounter_error,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Advance!
  #[named]
  pub fn advance(&mut self, scanner: &mut Scanner) -> Result<(), Error> {
    trace_enter!();
    self.previous = self.current;
    let mut error_messages = Vec::new();
    loop {
      match scanner.scan_token() {
        Ok(token) => {
          self.current = Some(token);
          break;
        },
        Err(error) => {
          self.did_encounter_error = Some(error.into());
          error_messages.push(error.to_string());
        },
      }
      self.current = Some(scanner.scan_token()?);
    }
    let result = match &self.did_encounter_error {
      Some(error) => {
        if error_messages.len() > 1 {
          Err(Error::MultipleErrors(error_messages))
        } else {
          Err(error.clone())
        }
      },
      None => Ok(()),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Consume.
  #[named]
  pub fn consume(&mut self, scanner: &mut Scanner, expected: TokenType, message: &str) -> Result<(), Error> {
    trace_enter!();
    trace_var!(expected);
    trace_var!(message);
    let current_type = self.current.unwrap().r#type;
    trace_var!(current_type);
    let result = if current_type == expected {
      self.advance(scanner)?;
      Ok(())
    } else {
      Err(Error::UnexpectedTokenError(current_type, message.to_string()))
    };
    trace_exit!();
    result
  }

  /// Expression.
  #[named]
  pub fn expression(&mut self) -> Result<(), Error> {
    trace_enter!();
    trace_exit!();
    Ok(())
  }

  /// Emit an instruction.
  #[named]
  #[inline]
  pub fn emit_instruction(&mut self, program: &mut Program, instruction: &Instruction) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    trace_var!(instruction);
    program
      .instructions
      .append(*instruction, self.previous.unwrap().line_number);
    trace_exit!();
    Ok(())
  }

  /// Emit an instruction.
  #[named]
  #[inline]
  pub fn emit_instructions(
    &mut self,
    program: &mut Program,
    instructions: (&Instruction, &Instruction),
  ) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    trace_var!(instructions);
    self.emit_instruction(program, instructions.0)?;
    self.emit_instruction(program, instructions.1)?;
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn emit_return(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.emit_instruction(program, &Instruction::Return)?;
    trace_exit!();
    Ok(())
  }
}
