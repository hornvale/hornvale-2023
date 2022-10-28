use std::mem::replace;

use crate::scripting_language::compiler::function_type::FunctionType;
use crate::scripting_language::compiler::Compiler;
use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
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
  pub compiler: Box<Compiler>,
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
    let function_name = garbage_collector.intern("script".to_owned());
    trace_var!(function_name);
    let compiler = Box::new(Compiler::new(function_name, FunctionType::Script));
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

  /// Compile!
  #[named]
  pub fn compile(mut self) -> Result<Reference<Function>, Error> {
    trace_enter!();
    self.advance()?;
    while !self.r#match(TokenType::Eof)? {
      self.parse_declaration()?;
    }
    self.emit_return()?;
    let result = self.garbage_collector.alloc(self.compiler.function);
    trace_var!(result);
    trace_exit!();
    Ok(result)
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
  pub fn parse_grouping(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    self.parse_expression()?;
    self.consume(TokenType::RightParenthesis, "expected ')' after expression")?;
    trace_exit!();
    Ok(())
  }

  /// Declaration.
  #[named]
  pub fn parse_declaration(&mut self) -> Result<(), Error> {
    trace_enter!();
    if self.r#match(TokenType::Function)? {
      self.parse_function_declaration()?;
    } else if self.r#match(TokenType::Var)? {
      self.parse_variable_declaration()?;
    } else {
      self.parse_statement()?;
    }
    if self.suppress_new_errors {
      self.synchronize()?;
    }
    trace_exit!();
    Ok(())
  }

  /// Function declaration.
  #[named]
  pub fn parse_function_declaration(&mut self) -> Result<(), Error> {
    trace_enter!();
    let function_index = self.parse_variable_identifier("expected a function name")?;
    trace_var!(function_index);
    self.mark_initialized()?;
    self.parse_function(FunctionType::Function)?;
    self.define_variable(function_index)?;
    trace_exit!();
    Ok(())
  }

  /// Variable declaration.
  #[named]
  pub fn parse_variable_declaration(&mut self) -> Result<(), Error> {
    trace_enter!();
    let variable_index = self.parse_variable_identifier("expected a variable identifier")?;
    trace_var!(variable_index);
    if self.r#match(TokenType::Equal)? {
      self.parse_expression()?;
    } else {
      self.emit_instruction(Instruction::Nil)?;
    }
    self.consume(TokenType::Semicolon, "expected ';' after variable declaration")?;
    self.define_variable(variable_index)?;
    trace_exit!();
    Ok(())
  }

  /// Statement.
  #[named]
  pub fn parse_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    if self.r#match(TokenType::Print)? {
      self.parse_print_statement()?;
    } else if self.r#match(TokenType::If)? {
      self.parse_if_statement()?;
    } else if self.r#match(TokenType::Return)? {
      self.parse_return_statement()?;
    } else if self.r#match(TokenType::While)? {
      self.parse_while_statement()?;
    } else if self.r#match(TokenType::For)? {
      self.parse_for_statement()?;
    } else if self.r#match(TokenType::LeftBrace)? {
      self.begin_scope()?;
      self.parse_block()?;
      self.end_scope()?;
    } else {
      self.parse_expression_statement()?;
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

  /// Function.
  #[named]
  pub fn parse_function(&mut self, function_type: FunctionType) -> Result<(), Error> {
    trace_enter!();
    self.push_compiler(function_type)?;
    self.begin_scope()?;
    self.consume(TokenType::LeftParenthesis, "expected '(' after function name.")?;
    if !self.check(TokenType::RightParenthesis) {
      loop {
        self.compiler.function.arity += 1;
        if self.compiler.function.arity > 255 {
          return Err(Error::FunctionArityExceededLimit(self.current));
        }
        let parameter = self.parse_variable_identifier("expected a parameter identifier")?;
        self.define_variable(parameter)?;
        if !self.r#match(TokenType::Comma)? {
          break;
        }
      }
    }
    self.consume(TokenType::RightParenthesis, "expected ')' after parameter list")?;
    self.consume(TokenType::LeftBrace, "expected '{' before function body")?;
    self.parse_block()?;
    let function = self.pop_compiler()?;
    let function_id = self.garbage_collector.alloc(function);
    let index = self.make_constant(Value::Function(function_id))?;
    self.emit_instruction(Instruction::Closure(index))?;
    trace_exit!();
    Ok(())
  }

  /// Block.
  #[named]
  pub fn parse_block(&mut self) -> Result<(), Error> {
    trace_enter!();
    while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
      self.parse_declaration()?;
    }
    self.consume(TokenType::RightBrace, "expected '}' after block")?;
    trace_exit!();
    Ok(())
  }

  /// End a scope.
  #[named]
  pub fn end_scope(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.compiler.depth -= 1;
    for i in (0..self.compiler.locals.len()).rev() {
      if self.compiler.locals[i].depth > self.compiler.depth {
        self.emit_instruction(Instruction::Pop)?;
        self.compiler.locals.pop();
      }
    }
    trace_exit!();
    Ok(())
  }

  /// Print statement.
  #[named]
  pub fn parse_print_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.parse_expression()?;
    self.consume(TokenType::Semicolon, "expected ';' after the expression")?;
    self.emit_instruction(Instruction::Print)?;
    trace_exit!();
    Ok(())
  }

  /// If statement.
  #[named]
  pub fn parse_if_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.r#match(TokenType::LeftParenthesis)?;
    self.parse_expression()?;
    self.r#match(TokenType::RightParenthesis)?;
    let then_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::JumpIfFalse(u16::MAX))?;
    self.emit_instruction(Instruction::Pop)?;
    self.parse_statement()?;
    let else_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::Jump(u16::MAX))?;
    self.patch_jump(then_jump as u16)?;
    self.emit_instruction(Instruction::Pop)?;
    if self.r#match(TokenType::Else)? {
      self.parse_statement()?;
    }
    self.patch_jump(else_jump as u16)?;
    trace_exit!();
    Ok(())
  }

  /// Return statement.
  #[named]
  pub fn parse_return_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    if let FunctionType::Script = self.compiler.function_type {
      // Not going to block `return` in top-level code ATM.
    } else if self.r#match(TokenType::Semicolon)? {
      self.emit_return()?;
    } else {
      self.parse_expression()?;
      self.consume(TokenType::Semicolon, "expected ';' after return value")?;
      self.emit_instruction(Instruction::Return)?;
    }
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
  pub fn patch_jump(&mut self, index: u16) -> Result<(), Error> {
    trace_enter!();
    trace_var!(index);
    let latest = self.compiler.function.chunk.instructions.instructions.len() as u16 - 1;
    trace_var!(latest);
    let offset = latest - index;
    trace_var!(offset);
    match self.compiler.function.chunk.instructions.instructions[index as usize] {
      Instruction::JumpIfFalse(ref mut dest) => *dest = offset,
      Instruction::Jump(ref mut dest) => *dest = offset,
      instruction => panic!("Incorrect instruction {:#?} at position", instruction),
    };
    trace_exit!();
    Ok(())
  }

  /// While statement.
  #[named]
  pub fn parse_while_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    let loop_start = self.compiler.function.chunk.instructions.instructions.len();
    self.r#match(TokenType::LeftParenthesis)?;
    self.parse_expression()?;
    self.r#match(TokenType::RightParenthesis)?;
    let exit_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::JumpIfFalse(u16::MAX))?;
    self.emit_instruction(Instruction::Pop)?;
    self.parse_statement()?;
    self.emit_loop(loop_start as u16)?;
    self.patch_jump(exit_jump as u16)?;
    self.emit_instruction(Instruction::Pop)?;
    trace_exit!();
    Ok(())
  }

  /// Emit a loop instruction.
  ///
  /// We're provided with the `index`, which is the location in code of the
  /// instruction to patch. We also have the current length of the code,
  /// which indicates how many instructions have been added since then.
  /// So we should take the difference of the two indices so that we jump back
  /// to when the condition is checked.
  #[named]
  pub fn emit_loop(&mut self, index: u16) -> Result<(), Error> {
    trace_enter!();
    trace_var!(index);
    let latest = self.compiler.function.chunk.instructions.instructions.len() as u16 - 1;
    trace_var!(latest);
    let offset = (latest - index) + 2;
    trace_var!(offset);
    self.emit_instruction(Instruction::Loop(offset))?;
    trace_exit!();
    Ok(())
  }

  /// Parse a function call.
  #[named]
  pub fn parse_call(&mut self, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(can_assign);
    let argument_count = self.parse_argument_list()?;
    trace_var!(argument_count);
    self.emit_instruction(Instruction::Call(argument_count))?;
    trace_exit!();
    Ok(())
  }

  /// Parse the argument length.
  #[named]
  pub fn parse_argument_list(&mut self) -> Result<u8, Error> {
    trace_enter!();
    let mut count: usize = 0;
    trace_var!(count);
    if !self.check(TokenType::RightParenthesis) {
      loop {
        self.parse_expression()?;
        if count == 255 {
          return Err(Error::FunctionCallArgumentsExceededLimit(self.current));
        }
        count += 1;
        if !self.r#match(TokenType::Comma)? {
          break;
        }
      }
    }
    self.consume(TokenType::RightParenthesis, "expected ')' after call arguments")?;
    let result = count as u8;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// For statement.
  #[named]
  pub fn parse_for_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.begin_scope()?;
    self.consume(TokenType::LeftParenthesis, "expected '(' after 'for'.")?;

    // Process initializer segment.
    if self.r#match(TokenType::Semicolon)? {
      // No initializer, no problem.
    } else if self.r#match(TokenType::Var)? {
      self.parse_variable_declaration()?;
    } else {
      self.parse_expression_statement()?;
    }

    let mut loop_start = self.compiler.function.chunk.instructions.instructions.len();

    // Process condition segment.
    let mut exit_jump: Option<usize> = None;
    if !self.r#match(TokenType::Semicolon)? {
      self.parse_expression()?;
      self.consume(TokenType::Semicolon, "expected ';' after loop condition.")?;
      exit_jump = Some(self.compiler.function.chunk.instructions.instructions.len());
      self.emit_instruction(Instruction::JumpIfFalse(0xFFFF))?;
      self.emit_instruction(Instruction::Pop)?;
    }

    // Process increment segment.
    if !self.r#match(TokenType::RightParenthesis)? {
      let body_jump = self.compiler.function.chunk.instructions.instructions.len();
      self.emit_instruction(Instruction::Jump(0xFFFF))?;
      let increment_start = self.compiler.function.chunk.instructions.instructions.len();
      self.parse_expression()?;
      self.emit_instruction(Instruction::Pop)?;
      self.consume(TokenType::RightParenthesis, "expected ')' after 'for' clauses.")?;
      self.emit_loop(loop_start as u16)?;
      loop_start = increment_start;
      self.patch_jump(body_jump as u16)?;
    }
    // Loop!
    self.parse_statement()?;
    self.emit_loop(loop_start as u16)?;
    if let Some(exit_jump) = exit_jump {
      self.patch_jump(exit_jump as u16)?;
      self.emit_instruction(Instruction::Pop)?;
    }
    self.end_scope()?;
    trace_exit!();
    Ok(())
  }

  /// Expression statement.
  #[named]
  pub fn parse_expression_statement(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.parse_expression()?;
    self.consume(TokenType::Semicolon, "expected ';' after the expression")?;
    self.emit_instruction(Instruction::Pop)?;
    trace_exit!();
    Ok(())
  }

  /// Expression.
  #[named]
  pub fn parse_expression(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.parse_precedence(Precedence::Assignment)?;
    trace_exit!();
    Ok(())
  }

  /// A number!
  #[named]
  #[inline]
  pub fn parse_number(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
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
    self.emit_constant(Value::Number(value))?;
    trace_exit!();
    Ok(())
  }

  /// A string!
  #[named]
  #[inline]
  pub fn parse_string(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
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
    self.emit_constant(Value::String(value))?;
    trace_exit!();
    Ok(())
  }

  /// Intern a string from the source.
  #[named]
  #[inline]
  pub fn intern_token(&mut self, token: &Token) -> Result<Reference<String>, Error> {
    trace_enter!();
    let start = token.start;
    trace_var!(start);
    let end = start + token.length;
    trace_var!(end);
    let string = &self.scanner.source[start..end];
    trace_var!(string);
    let result = self.garbage_collector.intern(string.to_owned());
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Parse a variable.
  #[named]
  pub fn parse_variable(&mut self, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    self.did_name_variable(self.previous.unwrap(), can_assign)?;
    trace_exit!();
    Ok(())
  }

  /// Parse an And.
  #[named]
  pub fn parse_and(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    let end_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::JumpIfFalse(u16::MAX))?;
    self.emit_instruction(Instruction::Pop)?;
    self.parse_precedence(Precedence::And)?;
    self.patch_jump(end_jump as u16)?;
    trace_exit!();
    Ok(())
  }

  /// Parse an Or.
  #[named]
  pub fn parse_or(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    let else_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::JumpIfFalse(u16::MAX))?;
    let end_jump = self.compiler.function.chunk.instructions.instructions.len();
    self.emit_instruction(Instruction::Jump(u16::MAX))?;
    self.patch_jump(else_jump as u16)?;
    self.emit_instruction(Instruction::Pop)?;
    self.parse_precedence(Precedence::Or)?;
    self.patch_jump(end_jump as u16)?;
    trace_exit!();
    Ok(())
  }

  /// Parse a variable identifier.
  #[named]
  pub fn parse_variable_identifier(&mut self, message: &str) -> Result<u16, Error> {
    trace_enter!();
    self.consume(TokenType::Identifier, message)?;
    self.declare_variable()?;
    if self.compiler.depth > 0 {
      return Ok(0);
    }
    let result = self.get_identifier_constant(self.previous.unwrap())?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Binary operator.
  #[named]
  pub fn parse_binary(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    let operator_type = self.previous.unwrap().r#type;
    let rule = self.get_rule(&operator_type);
    self.parse_precedence(rule.unwrap().precedence.next().unwrap())?;
    use TokenType::*;
    match operator_type {
      BangEqual => self.emit_instruction(Instruction::NotEqual)?,
      EqualEqual => self.emit_instruction(Instruction::Equal)?,
      GreaterThan => self.emit_instruction(Instruction::GreaterThan)?,
      GreaterThanOrEqual => self.emit_instruction(Instruction::GreaterThanOrEqual)?,
      LessThan => self.emit_instruction(Instruction::LessThan)?,
      LessThanOrEqual => self.emit_instruction(Instruction::LessThanOrEqual)?,
      Plus => self.emit_instruction(Instruction::Add)?,
      Minus => self.emit_instruction(Instruction::Subtract)?,
      Star => self.emit_instruction(Instruction::Multiply)?,
      Slash => self.emit_instruction(Instruction::Divide)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Unary operator.
  #[named]
  pub fn parse_unary(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    let operator_type = self.previous.unwrap().r#type;
    self.parse_expression()?;
    use TokenType::*;
    match operator_type {
      Minus => self.emit_instruction(Instruction::Negate)?,
      Bang => self.emit_instruction(Instruction::Not)?,
      _ => {},
    }
    trace_exit!();
    Ok(())
  }

  /// Literal.
  #[named]
  pub fn parse_literal(&mut self, _can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    let token_type = self.previous.unwrap().r#type;
    use TokenType::*;
    match token_type {
      True => self.emit_instruction(Instruction::True)?,
      False => self.emit_instruction(Instruction::False)?,
      Nil => self.emit_instruction(Instruction::Nil)?,
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
  pub fn declare_variable(&mut self) -> Result<(), Error> {
    trace_enter!();
    if self.compiler.depth == 0 {
      return Ok(());
    }
    let token = self.previous.unwrap();
    if self.compiler.has_local(self.scanner.source, &token) {
      return Err(Error::AttemptedToRedeclareVariable(Some(token)));
    }
    self.add_local(token)?;
    trace_exit!();
    Ok(())
  }

  /// Add a local variable.
  #[named]
  pub fn add_local(&mut self, token: Token) -> Result<(), Error> {
    trace_enter!();
    trace_var!(token);
    self.compiler.locals.push(Local::new(token, -1));
    trace_exit!();
    Ok(())
  }

  /// Define a variable.
  #[named]
  #[inline]
  pub fn define_variable(&mut self, index: u16) -> Result<(), Error> {
    trace_enter!();
    trace_var!(index);
    if self.compiler.depth > 0 {
      self.mark_initialized()?;
      return Ok(());
    }
    self.emit_instruction(Instruction::DefineGlobal(index))?;
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
  pub fn get_identifier_constant(&mut self, token: Token) -> Result<u16, Error> {
    trace_enter!();
    trace_var!(token);
    let reference = self.intern_token(&token)?;
    trace_var!(reference);
    let value = Value::String(reference);
    trace_var!(value);
    let result = self.make_constant(value)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Switch to a new compiler.
  #[named]
  pub fn push_compiler(&mut self, function_type: FunctionType) -> Result<(), Error> {
    trace_enter!();
    trace_var!(function_type);
    let function_name = self.intern_token(&self.previous.unwrap())?;
    trace_var!(function_name);
    let new_compiler = Box::new(Compiler::new(function_name, function_type));
    trace_var!(new_compiler);
    let old_compiler = replace(&mut self.compiler, new_compiler);
    self.compiler.enclosing = Some(old_compiler);
    trace_exit!();
    Ok(())
  }

  /// Pop the last compiler.
  #[named]
  pub fn pop_compiler(&mut self) -> Result<Function, Error> {
    trace_enter!();
    self.emit_return()?;
    let result = match self.compiler.enclosing.take() {
      Some(enclosing) => {
        let compiler = replace(&mut self.compiler, enclosing);
        compiler.function
      },
      None => return Err(Error::TriedToPopTopCompiler(self.current)),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Create a constant.
  #[named]
  #[inline]
  pub fn make_constant(&mut self, value: Value) -> Result<u16, Error> {
    trace_enter!();
    trace_var!(value);
    self.compiler.function.chunk.constants.push(value)?;
    let result = (self.compiler.function.chunk.constants.constants.len() - 1) as u16;
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
  pub fn emit_constant(&mut self, value: Value) -> Result<(), Error> {
    trace_enter!();
    trace_var!(value);
    let instruction = self.compiler.function.chunk.constants.push(value)?;
    self.emit_instruction(instruction)?;
    trace_exit!();
    Ok(())
  }

  /// Emit an instruction.
  #[named]
  #[inline]
  pub fn emit_instruction(&mut self, instruction: Instruction) -> Result<(), Error> {
    trace_enter!();
    trace_var!(instruction);
    self
      .compiler
      .function
      .chunk
      .instructions
      .append(instruction, self.previous.unwrap().line_number);
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn emit_return(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.emit_instruction(Instruction::Nil)?;
    self.emit_instruction(Instruction::Return)?;
    trace_exit!();
    Ok(())
  }

  /// Handle when we named a variable.
  #[named]
  pub fn did_name_variable(&mut self, name: Token, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(name);
    let get_op;
    let set_op;
    if let Some(index) = self.compiler.resolve_local(self.scanner.source, name)? {
      get_op = Instruction::GetLocal(index);
      set_op = Instruction::SetLocal(index);
    } else {
      let index = self.get_identifier_constant(name)?;
      get_op = Instruction::GetGlobal(index);
      set_op = Instruction::SetGlobal(index);
    }
    if can_assign && self.r#match(TokenType::Equal)? {
      self.parse_expression()?;
      self.emit_instruction(set_op)?;
    } else {
      self.emit_instruction(get_op)?;
    }
    trace_exit!();
    Ok(())
  }

  /// Parse precedence.
  #[named]
  pub fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), Error> {
    trace_enter!();
    trace_var!(precedence);
    self.advance()?;
    let previous_rule = self.get_previous_rule().unwrap();
    trace_var!(previous_rule);
    if previous_rule.prefix.is_none() {
      return Err(Error::ExpectedExpression(self.previous));
    }
    let prefix = previous_rule.prefix.unwrap();
    let can_assign = precedence <= Precedence::Assignment;
    prefix(self, can_assign)?;
    while precedence <= self.get_current_rule().unwrap().precedence {
      self.advance()?;
      let previous_rule = self.get_previous_rule().unwrap();
      if previous_rule.infix.is_none() {
        return Err(Error::ExpectedExpression(self.previous));
      }
      let infix = previous_rule.infix.unwrap();
      infix(self, can_assign)?;
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
