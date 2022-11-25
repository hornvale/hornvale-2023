use super::token::{Token, TokenType};
use super::ParserData;
use crate::command::*;
use anyhow::Error as AnyError;

/// The `Parser` type.
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
#[display(fmt = "input: {}, tokens: {:#?}", input, tokens)]
pub struct Parser<'input> {
  pub input: &'input str,
  pub tokens: Vec<Token<'input>>,
  pub current: usize,
}

impl<'input> Parser<'input> {
  /// Constructor.
  pub fn new(tokens: Vec<Token<'input>>, input: &'input str) -> Self {
    let current = 0;
    Self { input, tokens, current }
  }

  /// Parse input.
  pub fn parse(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.parse_command(data)
  }

  /// Parse a command.
  pub fn parse_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    if self.check(TokenType::Direction)? && self.peek_next()?.r#type == TokenType::Eof {
      return self.parse_direction_command(data);
    }
    if self.match_verb()? {
      self.match_noun_phrase(data)?;
      return self.parse_action_command(data);
    }
    self.parse_order_command(data)
  }

  /// Parse an order command; that is, tell another actor to do something.
  pub fn parse_order_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    if self.match_noun_phrase(data)? {
      self.consume(
        TokenType::Comma,
        &format!("Expected a comma after the addressee, {}", self.previous()?),
      )?;
    }
    Command::from_data(original_input, string, self.tokens.clone(), data)
  }

  /// Parse an action command; that is, a simple command.
  pub fn parse_action_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.advance()?;
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    Command::from_data(original_input, string, self.tokens.clone(), data)
  }

  /// Parse a direction command, e.g. "sw".
  pub fn parse_direction_command(&mut self, data: &impl ParserData) -> Result<Command, AnyError> {
    self.consume(TokenType::Direction, "Expected a valid direction.")?;
    let string = self.get_raw_arguments()?;
    let original_input = self.input.to_owned();
    let mut tokens = vec![Token {
      r#type: TokenType::Go,
      lexeme: "go",
      literal: None,
      entity_id: None,
    }];
    tokens.append(&mut self.tokens);
    Command::from_data(original_input, string, tokens, data)
  }

  /// Look for a match to a noun phrase.
  pub fn match_noun_phrase(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    while self.match_descriptor(data)? {
      self.current += 1;
    }
    if self.match_noun(data)? {
      self.current += 1;
    } else {
      return Ok(false);
    }
    Ok(true)
  }

  /// Look for a match to a descriptor.
  pub fn match_descriptor(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    if self.check(TokenType::Article)? {
      return Ok(true);
    }
    if self.check(TokenType::Genitive)? {
      return Ok(true);
    }
    let adjectives = data.get_adjectives()?;
    let lexeme = self.peek_next()?.lexeme;
    let next = self.current + 1;
    if adjectives.iter().any(|adjective| adjective == lexeme) {
      self.tokens[next].r#type = TokenType::Adjective;
      return Ok(true);
    }
    Ok(false)
  }

  /// Look for a match to a noun.
  pub fn match_noun(&mut self, data: &impl ParserData) -> Result<bool, AnyError> {
    let nouns = data.get_nouns()?;
    let lexeme = self.peek_next()?.lexeme;
    let next = self.current + 1;
    for (noun, entity_id) in nouns {
      if noun == lexeme {
        self.tokens[next].r#type = TokenType::Noun;
        self.tokens[next].entity_id = Some(entity_id);
        return Ok(true);
      }
    }
    Ok(false)
  }

  /// Look for a match to a verb.
  pub fn match_verb(&mut self) -> Result<bool, AnyError> {
    Ok(self.peek()?.is_verb())
  }

  /// Look for a match of the current token.
  pub fn r#match(&mut self, types: Vec<TokenType>) -> Result<bool, AnyError> {
    for r#type in types {
      if self.check(r#type)? {
        self.advance()?;
        return Ok(true);
      }
    }
    Ok(false)
  }

  /// Get the raw arguments to the command (for Echo, Eval, etc).
  pub fn get_raw_arguments(&self) -> Result<String, AnyError> {
    let result = self.tokens[1..]
      .iter()
      .map(|token| token.lexeme.to_string())
      .collect::<Vec<String>>()
      .join(" ");
    Ok(result)
  }

  /// Check that the next token is a specific type.
  pub fn check(&mut self, r#type: TokenType) -> Result<bool, AnyError> {
    if self.is_at_end()? {
      return Ok(false);
    }
    Ok(self.peek()?.r#type == r#type)
  }

  /// Advance to the next token.
  pub fn advance(&mut self) -> Result<Token, AnyError> {
    if !self.is_at_end()? {
      self.current += 1;
    }
    self.previous()
  }

  /// Have we processed all of the tokens?
  pub fn is_at_end(&self) -> Result<bool, AnyError> {
    Ok(self.peek()?.r#type == TokenType::Eof)
  }

  /// Peek at the current token.
  pub fn peek(&self) -> Result<Token, AnyError> {
    self.peek_at_offset(0)
  }

  /// Peek at the next token.
  pub fn peek_next(&self) -> Result<Token, AnyError> {
    self.peek_at_offset(1)
  }

  /// Peek at nth token.
  pub fn peek_at_offset(&self, offset: usize) -> Result<Token, AnyError> {
    match self.current + offset >= self.tokens.len() {
      true => Ok(Token {
        r#type: TokenType::Eof,
        lexeme: "",
        literal: None,
        entity_id: None,
      }),
      false => Ok(self.tokens[self.current + offset].clone()),
    }
  }

  /// Show the previous token.
  pub fn previous(&self) -> Result<Token, AnyError> {
    Ok(self.tokens.get(self.current - 1).unwrap().clone())
  }

  /// Consume the next token, or throw an error if it is not matched.
  pub fn consume<'error>(&mut self, r#type: TokenType, message: &'error str) -> Result<Token, AnyError> {
    if self.check(r#type)? {
      return self.advance();
    }
    bail!("{}", message);
  }

  /*
  #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
  pub enum Command {
    Echo(EchoCommand),
    Eval(EvalCommand),
    Idle(IdleCommand),
    GoDirection(GoDirectionCommand),
    LookAround(LookAroundCommand),
    LookAtEntity(LookAtEntityCommand),
    LookDirection(LookDirectionCommand),
    Quit(QuitCommand),
  }
  */

  /*
  #[named]
  pub fn declaration(&mut self) -> Result<Statement, Error> {
    if self.r#match(vec![Var]) {
      return self.var_declaration();
    }
    if self.r#match(vec![Function]) {
      return self.function_declaration("function");
    }
    self.statement()
  }

  #[named]
  pub fn var_declaration(&mut self) -> Result<Statement, Error> {
    match self.consume(Identifier, "Expected a variable name.") {
      Ok(name) => {
        let mut initializer = None;
        if self.r#match(vec![Equal]) {
          match self.expression() {
            Ok(expression) => initializer = Some(expression),
            Err(error) => {
              return Err(error);
            },
          }
        }
        self.consume(Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Statement::Variable { name, initializer })
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn function_declaration(&mut self, kind: &str) -> Result<Statement, Error> {
    let name = self.consume(Identifier, &format!("Expect {} name.", kind))?;
    self.consume(LeftParenthesis, &format!("Expect '(' after {} name.", kind))?;
    let mut parameters = Vec::new();
    if !self.check(RightParenthesis) {
      loop {
        if parameters.len() > 255 {
          return Err(Error::new(ErrorKind::Other, "Can't have more than 255 parameters."));
        }
        parameters.push(self.consume(Identifier, "Expect parameter name.")?);
        if !self.r#match(vec![Comma]) {
          break;
        }
      }
    }
    self.consume(RightParenthesis, "Expect ')' after parameters.")?;
    self.consume(LeftBrace, &format!("Expect '{{' before {} body.", kind))?;
    let body = Box::new(self.block()?);
    Ok(Statement::Function { name, parameters, body })
  }

  #[named]
  pub fn statement(&mut self) -> Result<Statement, Error> {
    if self.r#match(vec![TokenType::Print]) {
      return self.print_statement();
    }
    if self.r#match(vec![Return]) {
      return self.return_statement();
    }
    if self.r#match(vec![While]) {
      return self.while_statement();
    }
    if self.r#match(vec![For]) {
      return self.for_statement();
    }
    if self.r#match(vec![LeftBrace]) {
      return self.block();
    }
    if self.r#match(vec![If]) {
      return self.if_statement();
    }
    self.expression_statement()
  }

  #[named]
  pub fn expression(&mut self) -> Result<Expression, Error> {
    self.assignment()
  }

  #[named]
  pub fn assignment(&mut self) -> Result<Expression, Error> {
    let result = self.or_expression()?;
    if self.r#match(vec![Equal]) {
      let _equals = self.previous();
      let value = self.assignment()?;
      match result {
        Variable { identifier, .. } => Ok(Assignment {
          identifier,
          value: Box::new(value),
          scope_distance: None,
        }),
        _ => Err(Error::new(ErrorKind::Other, "Invalid assignment target.")),
      }
    } else {
      Ok(result)
    }
  }

  #[named]
  pub fn or_expression(&mut self) -> Result<Expression, Error> {
    let mut result = self.and_expression()?;
    while self.r#match(vec![Or]) {
      let operator = self.previous();
      let right = Box::new(self.and_expression()?);
      result = Logical {
        left: Box::new(result),
        operator,
        right,
      };
    }
    Ok(result)
  }

  #[named]
  pub fn and_expression(&mut self) -> Result<Expression, Error> {
    let mut result = self.equality()?;
    while self.r#match(vec![And]) {
      let operator = self.previous();
      let right = Box::new(self.equality()?);
      result = Logical {
        left: Box::new(result),
        operator,
        right,
      };
    }
    Ok(result)
  }

  #[named]
  pub fn equality(&mut self) -> Result<Expression, Error> {
    let mut result = self.comparison()?;
    while self.r#match(vec![BangEqual, EqualEqual]) {
      let operator = self.previous();
      let right = self.comparison()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn comparison(&mut self) -> Result<Expression, Error> {
    let mut result = self.term()?;
    while self.r#match(vec![GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual]) {
      let operator = self.previous();
      let right = self.term()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn term(&mut self) -> Result<Expression, Error> {
    let mut result = self.factor()?;
    while self.r#match(vec![Minus, Plus]) {
      let operator = self.previous();
      let right = self.factor()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn factor(&mut self) -> Result<Expression, Error> {
    let mut result = self.unary()?;
    while self.r#match(vec![Slash, Star]) {
      let operator = self.previous();
      let right = self.unary()?;
      result = Binary {
        left: Box::new(result),
        operator,
        right: Box::new(right),
      };
    }
    Ok(result)
  }

  #[named]
  pub fn unary(&mut self) -> Result<Expression, Error> {
    while self.r#match(vec![Bang, Minus]) {
      let operator = self.previous();
      let right = self.unary()?;
      return Ok(Unary {
        operator,
        right: Box::new(right),
      });
    }
    self.call()
  }

  #[named]
  pub fn call(&mut self) -> Result<Expression, Error> {
    let mut result = self.primary()?;
    loop {
      if self.r#match(vec![LeftParenthesis]) {
        result = self.finish_call(result)?;
      } else {
        break;
      }
    }
    Ok(result)
  }

  #[named]
  pub fn finish_call(&mut self, callee: Expression) -> Result<Expression, Error> {
    let mut arguments = Vec::new();
    if !self.check(RightParenthesis) {
      loop {
        arguments.push(self.expression()?);
        if !self.r#match(vec![Comma]) {
          break;
        }
      }
    }
    let closing_parenthesis = self.consume(RightParenthesis, "Expect ')' after arguments.")?;
    Ok(Expression::Call {
      callee: Box::new(callee),
      closing_parenthesis,
      arguments,
    })
  }

  #[named]
  pub fn primary(&mut self) -> Result<Expression, Error> {
    if self.r#match(vec![False]) {
      return Ok(Literal {
        value: Some(TokenLiteral::Boolean(false)),
      });
    }
    if self.r#match(vec![True]) {
      return Ok(Literal {
        value: Some(TokenLiteral::Boolean(true)),
      });
    }
    if self.r#match(vec![Nil]) {
      return Ok(Literal {
        value: Some(TokenLiteral::Nil),
      });
    }
    if self.r#match(vec![Number, String]) {
      return Ok(Literal {
        value: self.previous().literal,
      });
    }
    if self.r#match(vec![Identifier]) {
      return Ok(Variable {
        identifier: self.previous(),
        scope_distance: None,
      });
    }
    if self.r#match(vec![LeftParenthesis]) {
      let expression = self.expression()?;
      self.consume(RightParenthesis, "Expect ')' after expression.")?;
      return Ok(Grouping {
        expression: Box::new(expression),
      });
    }
    Err(Error::new(ErrorKind::Other, "Expected expression!"))
  }

  #[named]
  pub fn block(&mut self) -> Result<Statement, Error> {
    let mut statements = Vec::new();
    while !self.check(RightBrace) && !self.is_at_end() {
      statements.push(self.declaration()?);
    }
    self.consume(RightBrace, "Expect '}' after block.")?;
    Ok(Statement::Block(statements))
  }

  #[named]
  pub fn print_statement(&mut self) -> Result<Statement, Error> {
    let expression = self.expression();
    match expression {
      Ok(value) => {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Print(value))
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn return_statement(&mut self) -> Result<Statement, Error> {
    let token = self.previous();
    let mut expression = None;
    if !self.check(Semicolon) {
      expression = Some(self.expression()?);
    }
    self.consume(Semicolon, "Expect ';' after 'return' value.")?;
    Ok(Statement::Return { token, expression })
  }

  #[named]
  pub fn expression_statement(&mut self) -> Result<Statement, Error> {
    let expression = self.expression();
    match expression {
      Ok(value) => {
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Expression(value))
      },
      Err(error) => Err(error),
    }
  }

  #[named]
  pub fn if_statement(&mut self) -> Result<Statement, Error> {
    self.consume(LeftParenthesis, "Expect '(' after 'if'.")?;
    let condition = self.expression()?;
    self.consume(RightParenthesis, "Expect ')' after 'if' condition.")?;
    let then = Box::new(self.statement()?);
    let mut r#else = None;
    if self.r#match(vec![Else]) {
      r#else = Some(Box::new(self.statement()?));
    }
    Ok(Statement::If {
      condition,
      then,
      r#else,
    })
  }

  #[named]
  pub fn while_statement(&mut self) -> Result<Statement, Error> {
    self.consume(LeftParenthesis, "Expect '(' after 'while'.")?;
    let condition = self.expression()?;
    self.consume(RightParenthesis, "Expect ')' after 'while' condition.")?;
    let body = Box::new(self.statement()?);
    Ok(Statement::While { condition, body })
  }

  #[named]
  pub fn for_statement(&mut self) -> Result<Statement, Error> {
    self.consume(LeftParenthesis, "Expect '(' after 'for'.")?;
    let initializer = {
      if self.r#match(vec![Semicolon]) {
        None
      } else if self.r#match(vec![Var]) {
        Some(self.var_declaration()?)
      } else {
        Some(self.expression_statement()?)
      }
    };
    let condition = {
      if !self.check(Semicolon) {
        Some(self.expression()?)
      } else {
        None
      }
    };
    self.consume(Semicolon, "Expect ';' after 'for' loop condition.")?;
    let increment = {
      if !self.check(RightParenthesis) {
        Some(self.expression()?)
      } else {
        None
      }
    };
    self.consume(RightParenthesis, "Expect ')' after 'for' clauses.")?;
    let mut body = Box::new(self.statement()?);
    if let Some(increment_expression) = increment {
      body = Box::new(Statement::Block(vec![
        *body,
        Statement::Expression(increment_expression),
      ]));
    }
    if let Some(condition_expression) = condition {
      body = Box::new(Statement::While {
        condition: condition_expression,
        body,
      })
    }
    if let Some(initializer_expression) = initializer {
      body = Box::new(Statement::Block(vec![initializer_expression, *body]))
    }
    Ok(*body)
  }

  #[named]
  pub fn consume<'a>(&mut self, r#type: TokenType, message: &'a str) -> Result<Token, Error> {
    if self.check(r#type) {
      return Ok(self.advance());
    }
    self.parse_error(self.peek(), Error::new(ErrorKind::Other, message))
  }

  #[named]
  pub fn parse_error(&mut self, _token: Token, error: Error) -> Result<Token, Error> {
    Err(error)
  }

  #[named]
  pub fn synchronize(&mut self) {
    self.advance();
    while !self.is_at_end() {
      if self.previous().r#type == Semicolon {
        return;
      }
      match self.peek().r#type {
        Class | For | Function | If | TokenType::Print | Return | Var | While => return,
        _ => {},
      }
      self.advance();
    }
  }

  */
}
