use crate::scripting_language::chunk::Chunk;
use crate::scripting_language::compiler::Compiler;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::local::Local;
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
  /// The compiler for the current scope.
  pub compiler: Compiler,
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
    let compiler = Compiler::new();
    trace_var!(compiler);
    let rules = Rules::default();
    trace_var!(rules);
    let suppress_new_errors = false;
    trace_var!(suppress_new_errors);
    let did_encounter_error = None;
    trace_var!(did_encounter_error);
    let result = Self {
      scanner,
      garbage_collector,
      compiler,
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
  pub fn parse_grouping(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.parse_expression(chunk)?;
    self.consume(TokenType::RightParenthesis, "expected ')' after expression")?;
    trace_exit!();
    Ok(())
  }

  /// Declaration.
  #[named]
  pub fn parse_declaration(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    if self.r#match(TokenType::Var)? {
      self.parse_var_declaration(chunk)?;
    } else {
      self.parse_statement(chunk)?;
    }
    if self.suppress_new_errors {
      self.synchronize()?;
    }
    trace_exit!();
    Ok(())
  }

  /// Variable declaration.
  #[named]
  pub fn parse_var_declaration(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    let variable_index = self.parse_variable_identifier(chunk)?;
    trace_var!(variable_index);
    if self.r#match(TokenType::Equal)? {
      self.parse_expression(chunk)?;
    } else {
      self.emit_instruction(chunk, Instruction::Nil)?;
    }
    self.consume(TokenType::Semicolon, "expected ';' after variable declaration")?;
    self.define_variable(chunk, variable_index)?;
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_statement(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    if self.r#match(TokenType::Print)? {
      self.parse_print_statement(chunk)?;
    } else if self.r#match(TokenType::If)? {
      self.parse_if_statement(chunk)?;
    } else if self.r#match(TokenType::LeftBrace)? {
      self.begin_scope()?;
      self.parse_block(chunk)?;
      self.end_scope(chunk)?;
    } else {
      self.parse_expression_statement(chunk)?;
    }
    trace_exit!();
    Ok(())
  }

  /// Begin a scope.
  #[named]
  pub fn begin_scope(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.compiler.depth += 1;
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_block(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
      self.parse_declaration(chunk)?;
    }
    self.consume(TokenType::RightBrace, "expected '}' after block")?;
    trace_exit!();
    Ok(())
  }

  /// End a scope.
  #[named]
  pub fn end_scope(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    self.compiler.depth -= 1;
    for i in (0..self.compiler.locals.len()).rev() {
      if self.compiler.locals[i].depth > self.compiler.depth {
        self.emit_instruction(chunk, Instruction::Pop)?;
        self.compiler.locals.pop();
      }
    }
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_print_statement(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.parse_expression(chunk)?;
    self.consume(TokenType::Semicolon, "expected ';' after the expression")?;
    self.emit_instruction(chunk, Instruction::Print)?;
    trace_exit!();
    Ok(())
  }

  /// If statement.
  #[named]
  pub fn parse_if_statement(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    if self.r#match(TokenType::LeftParenthesis)? {
      self.consume(TokenType::LeftParenthesis, "expected '(' before 'if' expression")?;
    }
    self.parse_expression(chunk)?;
    if self.r#match(TokenType::RightParenthesis)? {
      self.consume(TokenType::RightParenthesis, "expected ')' after 'if' expression")?;
    }
    let then_jump = chunk.instructions.instructions.len();
    self.emit_instruction(chunk, Instruction::JumpIfFalse(u16::MAX))?;
    self.emit_instruction(chunk, Instruction::Pop)?;
    self.parse_statement(chunk)?;
    let else_jump = chunk.instructions.instructions.len();
    self.patch_jump(chunk, then_jump as u16)?;
    self.emit_instruction(chunk, Instruction::Jump(u16::MAX))?;
    self.emit_instruction(chunk, Instruction::Pop)?;
    if self.r#match(TokenType::Else)? {
      self.parse_statement(chunk)?;
    }
    self.patch_jump(chunk, else_jump as u16)?;
    trace_exit!();
    Ok(())
  }

  /// Patch the jump statement.
  ///
  /// We're provided with the `index`, which is the location in code of the
  /// instruction to patch. We also have the current length of the code,
  /// which indicates how many instructions have been added since then.
  /// So we should take the difference of the two indices and add one so that
  /// we jump cleanly to the next instruction.
  #[named]
  pub fn patch_jump(&mut self, chunk: &mut Chunk, index: u16) -> Result<(), Error> {
    trace_enter!();
    trace_var!(index);
    let latest = chunk.instructions.instructions.len() as u16 - 1;
    trace_var!(latest);
    let offset = (latest - index) + 1;
    trace_var!(offset);
    match chunk.instructions.instructions[index as usize] {
      Instruction::JumpIfFalse(ref mut dest) => *dest = offset,
      Instruction::Jump(ref mut dest) => *dest = offset,
      instruction => panic!("Incorrect instruction {:#?} at position", instruction),
    };
    trace_exit!();
    Ok(())
  }

  /// Expression statement.
  #[named]
  pub fn parse_expression_statement(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.parse_expression(chunk)?;
    self.consume(TokenType::Semicolon, "expected ';' after the expression")?;
    self.emit_instruction(chunk, Instruction::Pop)?;
    trace_exit!();
    Ok(())
  }

  /// Expression.
  #[named]
  pub fn parse_expression(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.parse_precedence(&Precedence::Assignment, chunk)?;
    trace_exit!();
    Ok(())
  }

  /// A number!
  #[named]
  #[inline]
  pub fn parse_number(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
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
    self.emit_constant(chunk, Value::Number(value))?;
    trace_exit!();
    Ok(())
  }

  /// A string!
  #[named]
  #[inline]
  pub fn parse_string(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
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
    self.emit_constant(chunk, Value::String(value))?;
    trace_exit!();
    Ok(())
  }

  /// Intern a string from the source.
  #[named]
  #[inline]
  pub fn intern_token(&mut self, token: &Token) -> Result<Value, Error> {
    trace_enter!();
    let start = token.start;
    trace_var!(start);
    let end = start + token.length;
    trace_var!(end);
    let string = &self.scanner.source[start..end];
    trace_var!(string);
    let value = self.garbage_collector.intern(string.to_owned());
    trace_var!(value);
    let result = Value::String(value);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Parse a variable.
  #[named]
  pub fn parse_variable(&mut self, chunk: &mut Chunk, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.did_name_variable(chunk, self.previous.unwrap(), can_assign)?;
    trace_exit!();
    Ok(())
  }

  /// Parse a variable identifier.
  #[named]
  pub fn parse_variable_identifier(&mut self, chunk: &mut Chunk) -> Result<u16, Error> {
    trace_enter!();
    trace_var!(chunk);
    self.consume(TokenType::Identifier, "expected a variable identifier")?;
    self.declare_variable(chunk)?;
    if self.compiler.depth > 0 {
      return Ok(0);
    }
    let result = self.get_identifier_constant(chunk, self.previous.unwrap())?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Binary operator.
  #[named]
  pub fn parse_binary(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    let operator_type = self.previous.unwrap().r#type;
    let rule = self.get_rule(&operator_type);
    self.parse_precedence(&rule.unwrap().precedence.next().unwrap(), chunk)?;
    use TokenType::*;
    match operator_type {
      BangEqual => self.emit_instruction(chunk, Instruction::NotEqual)?,
      EqualEqual => self.emit_instruction(chunk, Instruction::Equal)?,
      GreaterThan => self.emit_instruction(chunk, Instruction::GreaterThan)?,
      GreaterThanOrEqual => self.emit_instruction(chunk, Instruction::GreaterThanOrEqual)?,
      LessThan => self.emit_instruction(chunk, Instruction::LessThan)?,
      LessThanOrEqual => self.emit_instruction(chunk, Instruction::LessThanOrEqual)?,
      Plus => self.emit_instruction(chunk, Instruction::Add)?,
      Minus => self.emit_instruction(chunk, Instruction::Subtract)?,
      Star => self.emit_instruction(chunk, Instruction::Multiply)?,
      Slash => self.emit_instruction(chunk, Instruction::Divide)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Unary operator.
  #[named]
  pub fn parse_unary(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    let operator_type = self.previous.unwrap().r#type;
    self.parse_expression(chunk)?;
    use TokenType::*;
    match operator_type {
      Minus => self.emit_instruction(chunk, Instruction::Negate)?,
      Bang => self.emit_instruction(chunk, Instruction::Not)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Literal.
  #[named]
  pub fn parse_literal(&mut self, chunk: &mut Chunk, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    let token_type = self.previous.unwrap().r#type;
    use TokenType::*;
    match token_type {
      True => self.emit_instruction(chunk, Instruction::True)?,
      False => self.emit_instruction(chunk, Instruction::False)?,
      Nil => self.emit_instruction(chunk, Instruction::Nil)?,
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

  /// Declare a variable.
  #[named]
  #[inline]
  pub fn declare_variable(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    if self.compiler.depth == 0 {
      return Ok(());
    }
    let token = self.previous.unwrap();
    if self.compiler.has_local(self.scanner.source, &token) {
      return Err(Error::AttemptedToRedeclareVariable(Some(token)));
    }
    self.add_local(chunk, token)?;
    trace_exit!();
    Ok(())
  }

  /// Add a local variable.
  #[named]
  pub fn add_local(&mut self, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    trace_var!(token);
    self.compiler.locals.push(Local::new(token, -1));
    trace_exit!();
    Ok(())
  }

  /// Define a variable.
  #[named]
  #[inline]
  pub fn define_variable(&mut self, chunk: &mut Chunk, index: u16) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    trace_var!(index);
    if self.compiler.depth > 0 {
      self.mark_initialized()?;
      return Ok(());
    }
    self.emit_instruction(chunk, Instruction::DefineGlobal(index))?;
    trace_exit!();
    Ok(())
  }

  /// Mark the last global as initialized.
  #[named]
  pub fn mark_initialized(&mut self) -> Result<(), Error> {
    trace_enter!();
    if self.compiler.depth == 0 {
      return Ok(());
    }
    let last_local = self.compiler.locals.last_mut().unwrap();
    trace_var!(last_local);
    last_local.depth = self.compiler.depth;
    trace_exit!();
    Ok(())
  }

  /// Get an identifier constant.
  #[named]
  #[inline]
  pub fn get_identifier_constant(&mut self, chunk: &mut Chunk, token: Token) -> Result<u16, Error> {
    trace_enter!();
    trace_var!(chunk);
    trace_var!(token);
    let value = self.intern_token(&token)?;
    trace_var!(value);
    let result = self.make_constant(chunk, value)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Create a constant.
  #[named]
  #[inline]
  pub fn make_constant(&mut self, chunk: &mut Chunk, value: Value) -> Result<u16, Error> {
    trace_enter!();
    trace_var!(value);
    chunk.constants.push(value)?;
    let result = (chunk.constants.constants.len() - 1) as u16;
    trace_var!(result);
    trace_exit!();
    Ok(result)
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
  pub fn emit_constant(&mut self, chunk: &mut Chunk, value: Value) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    trace_var!(value);
    let instruction = chunk.constants.push(value)?;
    self.emit_instruction(chunk, instruction)?;
    trace_exit!();
    Ok(())
  }

  /// Emit an instruction.
  #[named]
  #[inline]
  pub fn emit_instruction(&mut self, chunk: &mut Chunk, instruction: Instruction) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    trace_var!(instruction);
    chunk
      .instructions
      .append(instruction, self.previous.unwrap().line_number);
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn emit_return(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(chunk);
    self.emit_instruction(chunk, Instruction::Return)?;
    trace_exit!();
    Ok(())
  }

  /// Handle when we named a variable.
  #[named]
  pub fn did_name_variable(&mut self, chunk: &mut Chunk, name: Token, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(name);
    let get_op;
    let set_op;
    if let Some(index) = self.compiler.resolve_local(self.scanner.source, name)? {
      get_op = Instruction::GetLocal(index);
      set_op = Instruction::SetLocal(index);
    } else {
      let index = self.get_identifier_constant(chunk, name)?;
      get_op = Instruction::GetGlobal(index);
      set_op = Instruction::SetGlobal(index);
    }
    if can_assign && self.r#match(TokenType::Equal)? {
      self.parse_expression(chunk)?;
      self.emit_instruction(chunk, set_op)?;
    } else {
      self.emit_instruction(chunk, get_op)?;
    }
    trace_exit!();
    Ok(())
  }

  /// Parse precedence.
  #[named]
  pub fn parse_precedence(&mut self, precedence: &Precedence, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(precedence);
    self.advance()?;
    let previous_rule = self.get_previous_rule().unwrap();
    trace_var!(previous_rule);
    if previous_rule.prefix.is_none() {
      return Err(Error::ExpectedExpression(self.previous));
    }
    let prefix = previous_rule.prefix.unwrap();
    let can_assign = precedence <= &Precedence::Assignment;
    prefix(self, chunk, can_assign)?;
    while precedence <= &self.get_current_rule().unwrap().precedence {
      self.advance()?;
      let previous_rule = self.get_previous_rule().unwrap();
      if previous_rule.infix.is_none() {
        return Err(Error::ExpectedExpression(self.previous));
      }
      let infix = previous_rule.infix.unwrap();
      infix(self, chunk, can_assign)?;
    }
    if can_assign && self.r#match(TokenType::Equal)? {
      return Err(Error::InvalidAssignmentTarget(self.previous));
    }
    trace_exit!();
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
