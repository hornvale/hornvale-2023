use std::collections::HashMap;

use crate::scripting_language::parser::rule::ParseFn;
use crate::scripting_language::parser::rule::Rule;
use crate::scripting_language::parser::Parser;
use crate::scripting_language::parser::Precedence;
use crate::scripting_language::token::r#type::Type as TokenType;

/// The `Rules` type.
#[derive(Clone, Debug, Display)]
#[display(fmt = "rules: {:#?}", rules)]
pub struct Rules<'source> {
  /// The individual rules.
  pub rules: HashMap<TokenType, Rule<'source>>,
}

impl<'source> Rules<'source> {
  /// Constructor.

  pub fn new() -> Self {
    let rules = HashMap::new();

    Self { rules }
  }

  /// Add a new rule.

  pub fn add_rule(
    &mut self,
    token_type: TokenType,
    prefix: Option<ParseFn<'source>>,
    infix: Option<ParseFn<'source>>,
    precedence: Precedence,
  ) {
    self.rules.insert(
      token_type,
      Rule {
        prefix,
        infix,
        precedence,
      },
    );
  }
}

impl<'source> Default for Rules<'source> {
  /// No constraints, just let it all hang out.

  fn default() -> Self {
    let mut result = Rules::new();

    use TokenType::*;
    result.add_rule(
      LeftParenthesis,
      Some(Parser::parse_grouping),
      Some(Parser::parse_call),
      Precedence::Call,
    );
    result.add_rule(RightParenthesis, None, None, Precedence::None);
    result.add_rule(LeftBrace, None, None, Precedence::None);
    result.add_rule(RightBrace, None, None, Precedence::None);
    result.add_rule(Comma, None, None, Precedence::None);
    result.add_rule(Dot, None, Some(Parser::parse_dot), Precedence::Call);
    result.add_rule(
      Minus,
      Some(Parser::parse_unary),
      Some(Parser::parse_binary),
      Precedence::Term,
    );
    result.add_rule(Plus, None, Some(Parser::parse_binary), Precedence::Term);
    result.add_rule(Semicolon, None, None, Precedence::None);
    result.add_rule(Slash, None, Some(Parser::parse_binary), Precedence::Factor);
    result.add_rule(Star, None, Some(Parser::parse_binary), Precedence::Factor);
    result.add_rule(Bang, Some(Parser::parse_unary), None, Precedence::None);
    result.add_rule(BangEqual, None, Some(Parser::parse_binary), Precedence::Equality);
    result.add_rule(Equal, None, None, Precedence::None);
    result.add_rule(EqualEqual, None, Some(Parser::parse_binary), Precedence::Equality);
    result.add_rule(GreaterThan, None, Some(Parser::parse_binary), Precedence::Comparison);
    result.add_rule(
      GreaterThanOrEqual,
      None,
      Some(Parser::parse_binary),
      Precedence::Comparison,
    );
    result.add_rule(LessThan, None, Some(Parser::parse_binary), Precedence::Comparison);
    result.add_rule(
      LessThanOrEqual,
      None,
      Some(Parser::parse_binary),
      Precedence::Comparison,
    );
    result.add_rule(Identifier, Some(Parser::parse_variable), None, Precedence::None);
    result.add_rule(String, Some(Parser::parse_string), None, Precedence::None);
    result.add_rule(Number, Some(Parser::parse_number), None, Precedence::None);
    result.add_rule(And, None, Some(Parser::parse_and), Precedence::And);
    result.add_rule(Class, None, None, Precedence::None);
    result.add_rule(Else, None, None, Precedence::None);
    result.add_rule(False, Some(Parser::parse_literal), None, Precedence::None);
    result.add_rule(For, None, None, Precedence::None);
    result.add_rule(Function, None, None, Precedence::None);
    result.add_rule(If, None, None, Precedence::None);
    result.add_rule(Nil, Some(Parser::parse_literal), None, Precedence::None);
    result.add_rule(Or, None, Some(Parser::parse_or), Precedence::Or);
    result.add_rule(Print, None, None, Precedence::None);
    result.add_rule(Return, None, None, Precedence::None);
    result.add_rule(Super, Some(Parser::parse_super), None, Precedence::None);
    result.add_rule(This, Some(Parser::parse_this), None, Precedence::None);
    result.add_rule(True, Some(Parser::parse_literal), None, Precedence::None);
    result.add_rule(Var, None, None, Precedence::None);
    result.add_rule(While, None, None, Precedence::None);
    result.add_rule(Eof, None, None, Precedence::None);

    result
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_rules() {
    init();
    let _rules = Rules::default();
  }
}
