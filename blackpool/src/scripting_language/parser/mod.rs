use std::mem::replace;

use crate::scripting_language::class_compiler::ClassCompiler;
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
  pub compiler: Box<Compiler<'source>>,
  /// The class compiler.
  pub class_compiler: Option<Box<ClassCompiler>>,
  /// The current token.
  pub current: Option<Token<'source>>,
  /// The last token processed.
  pub previous: Option<Token<'source>>,
  /// The rules!
  pub rules: Rules<'source>,
  /// Whether we should suppress new errors ("Panic Mode").
  pub suppress_new_errors: bool,
  /// Whether we have actually encountered an error.
  pub did_encounter_error: bool,
  /// Errors that arise in the process of resolving locals and upvalues.
  pub resolver_errors: Vec<&'static str>,
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
    let did_encounter_error = false;
    trace_var!(did_encounter_error);
    let resolver_errors = Vec::new();
    trace_var!(resolver_errors);
    let class_compiler = None;
    trace_var!(class_compiler);
    let result = Self {
      scanner,
      garbage_collector,
      compiler,
      current,
      previous,
      rules,
      suppress_new_errors,
      did_encounter_error,
      resolver_errors,
      class_compiler,
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
    let mut first_error = None;
    while !self.r#match(TokenType::Eof)? {
      if let (None, Err(error)) = (&first_error, self.parse_declaration()) {
        first_error = Some(error);
      }
    }
    self.emit_return()?;
    let result = match self.did_encounter_error {
      false => Ok(self.garbage_collector.alloc(self.compiler.function)),
      true => Err(first_error.unwrap()),
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
    loop {
      self.current = Some(self.scanner.scan_token()?);
      if let TokenType::Error = self.current.unwrap().r#type {
        self.did_encounter_error_at_current(self.current.unwrap().lexeme);
      } else {
        break;
      }
    }
    Ok(())
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
      self.did_encounter_error_at_current(message);
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
    if self.r#match(TokenType::Class)? {
      self.parse_class_declaration()?;
    } else if self.r#match(TokenType::Function)? {
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

  /// Class declaration.
  #[named]
  pub fn parse_class_declaration(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.consume(TokenType::Identifier, "Expect class name.")?;
    let class_name = self.previous.unwrap();
    trace_var!(class_name);
    let name_constant = self.get_identifier_constant(class_name)?;
    trace_var!(name_constant);
    self.declare_variable()?;
    self.emit_instruction(Instruction::Class(name_constant))?;
    self.define_variable(name_constant)?;
    let old_class_compiler = self.class_compiler.take();
    let new_class_compiler = ClassCompiler::new(old_class_compiler);
    self.class_compiler.replace(new_class_compiler);
    if self.r#match(TokenType::LessThan)? {
      self.consume(TokenType::Identifier, "Expect superclass name.")?;
      self.parse_variable(false)?;
      if class_name.lexeme == self.previous.unwrap().lexeme {
        self.did_encounter_error("A class can't inherit from itself.");
        return Err(Error::AttemptedToDeclareClassAsASubclassOfItself);
      }
      self.begin_scope()?;
      self.add_local(Token::synthesize("super"))?;
      self.define_variable(0)?;
      self.did_name_variable(class_name, false)?;
      self.emit_instruction(Instruction::Inherit)?;
      self.class_compiler.as_mut().unwrap().has_superclass = true;
    }
    self.did_name_variable(class_name, false)?;
    self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
    while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
      self.parse_method_declaration()?;
    }
    self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
    self.emit_instruction(Instruction::Pop)?;
    if self.class_compiler.as_ref().unwrap().has_superclass {
      self.end_scope()?;
    }
    match self.class_compiler.take() {
      Some(class_compiler) => self.class_compiler = class_compiler.enclosing,
      None => self.class_compiler = None,
    }
    trace_exit!();
    Ok(())
  }

  /// Method declaration.
  #[named]
  pub fn parse_method_declaration(&mut self) -> Result<(), Error> {
    trace_enter!();
    self.consume(TokenType::Identifier, "Expect method name.")?;
    let constant = self.get_identifier_constant(self.previous.unwrap())?;
    trace_var!(constant);
    let function_type = if self.previous.unwrap().lexeme == "init" {
      FunctionType::Initializer
    } else {
      FunctionType::Method
    };
    self.parse_function(function_type)?;
    self.emit_instruction(Instruction::Method(constant))?;
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
    let variable_index = self.parse_variable_identifier("Expect variable name.")?;
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
          self.did_encounter_error_at_current("Can't have more than 255 parameters.");
          return Err(Error::FunctionArityExceededLimit);
        }
        let parameter = self.parse_variable_identifier("expected a parameter identifier")?;
        self.define_variable(parameter)?;
        if !self.r#match(TokenType::Comma)? {
          break;
        }
      }
    }
    self.consume(TokenType::RightParenthesis, "Expect ')' after parameters.")?;
    self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
    self.parse_block()?;
    let function = self.pop_compiler()?;
    let function_id = self.garbage_collector.alloc(function);
    let index = self.make_constant(Value::Function(function_id))?;
    self.emit_instruction(Instruction::Closure(index))?;
    trace_exit!();
    Ok(())
  }

  /// Encountered an error at the previous token.
  #[named]
  pub fn did_encounter_error(&mut self, message: &str) {
    trace_enter!();
    trace_var!(message);
    self.did_encounter_error_at_token(self.previous.unwrap(), message);
    trace_exit!();
  }

  /// Encountered an error at the current token.
  #[named]
  pub fn did_encounter_error_at_current(&mut self, message: &str) {
    trace_enter!();
    trace_var!(message);
    self.did_encounter_error_at_token(self.current.unwrap(), message);
    trace_exit!();
  }

  /// Encountered an error.
  #[named]
  pub fn did_encounter_error_at_token(&mut self, token: Token, message: &str) {
    trace_enter!();
    trace_var!(token);
    trace_var!(message);
    if self.suppress_new_errors {
      return;
    }
    self.did_encounter_error = true;
    self.suppress_new_errors = true;
    eprint!("[line {}] Error", token.line_number);
    use TokenType::*;
    match token.r#type {
      Error => (),
      Eof => eprint!(" at end"),
      _ => eprint!(" at '{}'", token.lexeme),
    };
    eprintln!(": {}", message);
    trace_exit!();
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

  /// Parse `super`.
  #[named]
  pub fn parse_super(&mut self, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(can_assign);
    if let Some(current_class) = self.class_compiler.as_ref() {
      if !current_class.has_superclass {
        self.did_encounter_error("Can't use 'super' in a class with no superclass.");
        return Err(Error::AttemptedToUseSuperInBaseClass);
      }
    } else {
      self.did_encounter_error("Can't use 'super' outside of a class.");
      return Err(Error::AttemptedToUseSuperOutsideClass);
    }
    self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
    self.consume(TokenType::Identifier, "Expect superclass method name.")?;
    let name = self.get_identifier_constant(self.previous.unwrap())?;
    self.did_name_variable(Token::synthesize("this"), false)?;
    if self.r#match(TokenType::LeftParenthesis)? {
      let argument_count = self.parse_argument_list()?;
      self.did_name_variable(Token::synthesize("super"), false)?;
      self.emit_instruction(Instruction::SuperInvoke((name, argument_count)))?;
    } else {
      self.did_name_variable(Token::synthesize("super"), false)?;
      self.emit_instruction(Instruction::GetSuper(name))?;
    }
    trace_exit!();
    Ok(())
  }

  /// Dots for method calls, etc.
  #[named]
  pub fn parse_dot(&mut self, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(can_assign);
    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
    let name = self.get_identifier_constant(self.previous.unwrap())?;
    if can_assign && self.r#match(TokenType::Equal)? {
      self.parse_expression()?;
      self.emit_instruction(Instruction::SetProperty(name))?;
    } else if self.r#match(TokenType::LeftParenthesis)? {
      let argument_count = self.parse_argument_list()?;
      self.emit_instruction(Instruction::Invoke((name, argument_count)))?;
    } else {
      self.emit_instruction(Instruction::GetProperty(name))?;
    }
    trace_exit!();
    Ok(())
  }

  /// Parse `this`.
  #[named]
  pub fn parse_this(&mut self, can_assign: bool) -> Result<(), Error> {
    trace_enter!();
    trace_var!(can_assign);
    if self.class_compiler.is_none() {
      self.did_encounter_error("Can't use 'this' outside of a class.");
      return Err(Error::AttemptedToUseThisOutsideClass);
    }
    self.parse_variable(false)?;
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
        if self.compiler.locals[i].is_captured {
          self.emit_instruction(Instruction::CloseUpvalue)?;
        } else {
          self.emit_instruction(Instruction::Pop)?;
        }
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
    self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
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
      self.did_encounter_error("Can't return from top-level code.");
    } else if self.r#match(TokenType::Semicolon)? {
      self.emit_return()?;
    } else {
      if let FunctionType::Initializer = self.compiler.function_type {
        self.did_encounter_error("Can't return a value from an initializer.");
      }
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
          self.did_encounter_error("Can't have more than 255 arguments.");
          return Err(Error::FunctionCallArgumentsExceededLimit);
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
    self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
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
    let string = previous.lexeme;
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
    let string = &previous.lexeme[1..(previous.lexeme.len() - 1)];
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
    let string = token.lexeme;
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
    self.parse_precedence(rule.unwrap().precedence.next())?;
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
    self.parse_precedence(Precedence::Unary)?;
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
    if self.compiler.has_local(&token) {
      self.did_encounter_error("Already variable with this name in this scope.");
      return Err(Error::AttemptedToRedeclareVariable);
    }
    self.add_local(token)?;
    trace_exit!();
    Ok(())
  }

  /// Add a local variable.
  #[named]
  pub fn add_local(&mut self, token: Token<'source>) -> Result<(), Error> {
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
      None => {
        return Err(Error::TriedToPopTopCompiler);
      },
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
    match self.compiler.function_type {
      FunctionType::Initializer => self.emit_instruction(Instruction::GetLocal(0))?,
      _ => self.emit_instruction(Instruction::Nil)?,
    };
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
    if let Some(index) = self.resolve_local(name) {
      get_op = Instruction::GetLocal(index);
      set_op = Instruction::SetLocal(index);
    } else if let Some(index) = self.resolve_upvalue(name) {
      get_op = Instruction::GetUpvalue(index);
      set_op = Instruction::SetUpvalue(index);
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
      self.did_encounter_error("Expect expression.");
      return Err(Error::ExpectedExpression);
    }
    let prefix = previous_rule.prefix.unwrap();
    let can_assign = precedence <= Precedence::Assignment;
    prefix(self, can_assign)?;
    while self.is_lower_precedence(precedence) {
      self.advance()?;
      let previous_rule = self.get_previous_rule().unwrap();
      let infix = previous_rule.infix.unwrap();
      infix(self, can_assign)?;
    }
    if can_assign && self.r#match(TokenType::Equal)? {
      self.did_encounter_error("Invalid assignment target.");
      return Err(Error::InvalidAssignmentTarget);
    }
    trace_exit!();
    Ok(())
  }

  /// Is this current operation lower precedence?
  #[named]
  pub fn is_lower_precedence(&self, precedence: Precedence) -> bool {
    trace_enter!();
    trace_var!(precedence);
    let current_precedence = self.get_current_rule().unwrap().precedence;
    trace_var!(current_precedence);
    let result = precedence <= current_precedence;
    trace_var!(result);
    trace_exit!();
    result
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

  /// Resolve a local reference.
  #[named]
  pub fn resolve_local(&mut self, token: Token) -> Option<u16> {
    trace_enter!();
    trace_var!(token);
    let result = self.compiler.resolve_local(token, &mut self.resolver_errors);
    while let Some(error) = self.resolver_errors.pop() {
      self.did_encounter_error(error);
    }
    result
  }

  /// Resolve an upvalue.
  #[named]
  pub fn resolve_upvalue(&mut self, token: Token) -> Option<u16> {
    trace_enter!();
    trace_var!(token);
    let result = self.compiler.resolve_upvalue(token, &mut self.resolver_errors);
    while let Some(error) = self.resolver_errors.pop() {
      self.did_encounter_error(error);
    }
    trace_var!(result);
    trace_exit!();
    result
  }
}
