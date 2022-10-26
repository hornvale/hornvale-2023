use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::program::Program;
use crate::scripting_language::scanner::Scanner;
use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;
use crate::scripting_language::value::Value;

pub mod error;
use error::Error;
pub mod precedence;
use precedence::Precedence;
pub mod rule;
use rule::Rule;
pub mod rules;
use rules::Rules;

/// The `Parser` type.
#[derive(Debug, Display)]
#[display(
  fmt = "scanner: {}, current: {:#?}, previous: {:#?}, rules: {}, suppress_new_errors: {}, did_encounter_error: {:#?}",
  scanner,
  current,
  previous,
  rules,
  suppress_new_errors,
  did_encounter_error
)]
pub struct Parser<'source> {
  /// The scanner.
  pub scanner: Scanner<'source>,
  /// The garbage collector.
  pub garbage_collector: &'source mut GarbageCollector,
  /// The current token.
  pub current: Option<Token>,
  /// The last token processed.
  pub previous: Option<Token>,
  /// The rules!
  pub rules: Rules<'source>,
  /// Whether we should suppress new errors ("Panic Mode").
  pub suppress_new_errors: bool,
  /// Whether we have actually encountered an error.
  pub did_encounter_error: Option<Error>,
}

impl<'source> Parser<'source> {
  /// Constructor.
  #[named]
  pub fn new(scanner: Scanner<'source>, garbage_collector: &'source mut GarbageCollector) -> Parser<'source> {
    trace_enter!();
    trace_var!(scanner);
    trace_var!(garbage_collector);
    let current = None;
    trace_var!(current);
    let previous = None;
    trace_var!(previous);
    let rules = Rules::default();
    trace_var!(rules);
    let suppress_new_errors = false;
    trace_var!(suppress_new_errors);
    let did_encounter_error = None;
    trace_var!(did_encounter_error);
    let result = Self {
      scanner,
      garbage_collector,
      current,
      previous,
      rules,
      suppress_new_errors,
      did_encounter_error,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Advance!
  #[named]
  pub fn advance(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.previous = self.current;
    let mut error_messages = Vec::new();
    loop {
      match self.scanner.scan_token() {
        Ok(token) => {
          self.current = Some(token);
          break;
        },
        Err(error) => {
          self.did_encounter_error = Some(error.into());
          error_messages.push(error.to_string());
        },
      }
      self.current = Some(self.scanner.scan_token()?);
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
  pub fn consume(&mut self, expected: TokenType, message: &str) -> Result<(), Error> {
    trace_enter!();
    trace_var!(expected);
    trace_var!(message);
    let current_type = self.current.unwrap().r#type;
    trace_var!(current_type);
    let result = if current_type == expected {
      self.advance()?;
      Ok(())
    } else {
      Err(Error::UnexpectedTokenError(current_type, message.to_string()))
    };
    trace_exit!();
    result
  }

  /// Grouping.
  #[named]
  pub fn parse_grouping(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.parse_expression(program)?;
    self.consume(TokenType::RightParenthesis, "expected ')' after expression")?;
    trace_exit!();
    Ok(())
  }

  /// Declaration.
  #[named]
  pub fn parse_declaration(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.parse_statement(program)?;
    if self.suppress_new_errors {
      self.synchronize()?;
    }
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_statement(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    if self.r#match(TokenType::Print)? {
      self.parse_print_statement(program)?;
    } else {
      self.parse_expression_statement(program)?;
    }
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_print_statement(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.parse_expression(program)?;
    self.consume(TokenType::Semicolon, "expected a semicolon after the expression")?;
    self.emit_instruction(program, Instruction::Print)?;
    trace_exit!();
    Ok(())
  }

  /// Expression statement.
  #[named]
  pub fn parse_expression_statement(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.parse_expression(program)?;
    self.consume(TokenType::Semicolon, "expected a semicolon after the expression")?;
    self.emit_instruction(program, Instruction::Pop)?;
    trace_exit!();
    Ok(())
  }

  /// Expression.
  #[named]
  pub fn parse_expression(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.parse_precedence(&Precedence::Assignment, program)?;
    trace_exit!();
    Ok(())
  }

  /// A number!
  #[named]
  #[inline]
  pub fn parse_number(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    let previous = self.previous.unwrap();
    trace_var!(previous);
    let start = previous.start;
    trace_var!(start);
    let end = start + previous.length;
    trace_var!(end);
    let string = &self.scanner.source[start..end];
    trace_var!(string);
    let value = string.parse::<f64>()?;
    trace_var!(value);
    self.emit_constant(program, Value::Number(value))?;
    trace_exit!();
    Ok(())
  }

  /// A string!
  #[named]
  #[inline]
  pub fn parse_string(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    let previous = self.previous.unwrap();
    trace_var!(previous);
    let start = previous.start + 1;
    trace_var!(start);
    let end = start + previous.length - 2;
    trace_var!(end);
    let string = &self.scanner.source[start..end];
    trace_var!(string);
    let value = self.garbage_collector.intern(string.to_owned());
    trace_var!(value);
    self.emit_constant(program, Value::String(value))?;
    trace_exit!();
    Ok(())
  }

  /// Binary operator.
  #[named]
  pub fn parse_binary(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    let operator_type = self.previous.unwrap().r#type;
    let rule = self.get_rule(&operator_type);
    self.parse_precedence(&rule.unwrap().precedence.next().unwrap(), program)?;
    use TokenType::*;
    match operator_type {
      BangEqual => self.emit_instruction(program, Instruction::NotEqual)?,
      EqualEqual => self.emit_instruction(program, Instruction::Equal)?,
      GreaterThan => self.emit_instruction(program, Instruction::GreaterThan)?,
      GreaterThanOrEqual => self.emit_instruction(program, Instruction::GreaterThanOrEqual)?,
      LessThan => self.emit_instruction(program, Instruction::LessThan)?,
      LessThanOrEqual => self.emit_instruction(program, Instruction::LessThanOrEqual)?,
      Plus => self.emit_instruction(program, Instruction::Add)?,
      Minus => self.emit_instruction(program, Instruction::Subtract)?,
      Star => self.emit_instruction(program, Instruction::Multiply)?,
      Slash => self.emit_instruction(program, Instruction::Divide)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Unary operator.
  #[named]
  pub fn parse_unary(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    let operator_type = self.previous.unwrap().r#type;
    self.parse_expression(program)?;
    use TokenType::*;
    match operator_type {
      Minus => self.emit_instruction(program, Instruction::Negate)?,
      Bang => self.emit_instruction(program, Instruction::Not)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Literal.
  #[named]
  pub fn parse_literal(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    let token_type = self.previous.unwrap().r#type;
    use TokenType::*;
    match token_type {
      True => self.emit_instruction(program, Instruction::True)?,
      False => self.emit_instruction(program, Instruction::False)?,
      Nil => self.emit_instruction(program, Instruction::Nil)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Rejoin society.
  #[named]
  pub fn synchronize(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.suppress_new_errors = false;
    while self.previous.unwrap().r#type != TokenType::Eof {
      if self.previous.unwrap().r#type == TokenType::Semicolon {
        return Ok(());
      }
      match self.current.unwrap().r#type {
        TokenType::Class
        | TokenType::Function
        | TokenType::Var
        | TokenType::For
        | TokenType::If
        | TokenType::While
        | TokenType::Print
        | TokenType::Return => return Ok(()),
        _ => (),
      }
      self.advance()?;
    }
    trace_exit!();
    Ok(())
  }

  /// Match current token.
  #[named]
  pub fn r#match(&mut self, token_type: TokenType) -> Result<bool, Error> {
    trace_enter!();
    trace_var!(token_type);
    if !self.check(token_type) {
      return Ok(false);
    }
    self.advance()?;
    let result = true;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Check type of current token.
  #[named]
  pub fn check(&mut self, token_type: TokenType) -> bool {
    trace_enter!();
    trace_var!(token_type);
    let result = self.current.is_some() && self.current.unwrap().r#type == token_type;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Emit a constant.
  #[named]
  #[inline]
  pub fn emit_constant(&mut self, program: &mut Program, value: Value) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    trace_var!(value);
    let instruction = program.constants.push(value)?;
    self.emit_instruction(program, instruction)?;
    trace_exit!();
    Ok(())
  }

  /// Emit an instruction.
  #[named]
  #[inline]
  pub fn emit_instruction(&mut self, program: &mut Program, instruction: Instruction) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    trace_var!(instruction);
    program
      .instructions
      .append(instruction, self.previous.unwrap().line_number);
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn emit_return(&mut self, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(program);
    self.emit_instruction(program, Instruction::Return)?;
    trace_exit!();
    Ok(())
  }

  /// Parse precedence.
  #[named]
  pub fn parse_precedence(&mut self, precedence: &Precedence, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(precedence);
    self.advance()?;
    let previous_rule = self.get_previous_rule().unwrap();
    trace_var!(previous_rule);
    if previous_rule.prefix.is_none() {
      return Err(Error::ExpectedExpression(self.previous));
    }
    let prefix = previous_rule.prefix.unwrap();
    prefix(self, program)?;
    while precedence <= &self.get_current_rule().unwrap().precedence {
      self.advance()?;
      let previous_rule = self.get_previous_rule().unwrap();
      if previous_rule.infix.is_none() {
        return Err(Error::ExpectedExpression(self.previous));
      }
      let infix = previous_rule.infix.unwrap();
      infix(self, program)?;
    }
    Ok(())
  }

  /// Get the previous rule.
  #[named]
  pub fn get_previous_rule(&self) -> Option<Rule<'source>> {
    trace_enter!();
    let result = match self.previous {
      None => None,
      Some(token) => self.get_rule(&token.r#type),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get the current rule.
  #[named]
  pub fn get_current_rule(&self) -> Option<Rule<'source>> {
    trace_enter!();
    let result = match self.current {
      None => None,
      Some(token) => self.get_rule(&token.r#type),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get a rule.
  #[named]
  pub fn get_rule(&self, token_type: &TokenType) -> Option<Rule<'source>> {
    trace_enter!();
    trace_var!(token_type);
    let result = self.rules.rules.get(token_type).cloned();
    trace_var!(result);
    trace_exit!();
    result
  }
}
