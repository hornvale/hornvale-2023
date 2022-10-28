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
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let rules = HashMap::new();
    trace_var!(rules);
    let result = Self { rules };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Add a new rule.
  #[named]
  pub fn add_rule(
    &mut self,
    token_type: TokenType,
    prefix: Option<ParseFn<'source>>,
    infix: Option<ParseFn<'source>>,
    precedence: Precedence,
  ) {
    trace_enter!();
    trace_var!(token_type);
    trace_var!(precedence);
    self.rules.insert(
      token_type,
      Rule {
        prefix,
        infix,
        precedence,
      },
    );
    trace_exit!();
  }
}

impl<'source> Default for Rules<'source> {
  /// No constraints, just let it all hang out.
  #[named]
  fn default() -> Self {
    trace_enter!();
    let mut result = Rules::new();
    trace_var!(result);
    use TokenType::*;
    result.add_rule(LeftParenthesis, Some(Parser::parse_grouping), None, Precedence::None);
    result.add_rule(RightParenthesis, None, None, Precedence::None);
    result.add_rule(LeftBrace, None, None, Precedence::None);
    result.add_rule(RightBrace, None, None, Precedence::None);
    result.add_rule(Comma, None, None, Precedence::None);
    result.add_rule(Dot, None, None, Precedence::None);
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
    result.add_rule(Super, None, None, Precedence::None);
    result.add_rule(This, None, None, Precedence::None);
    result.add_rule(True, Some(Parser::parse_literal), None, Precedence::None);
    result.add_rule(Var, None, None, Precedence::None);
    result.add_rule(While, None, None, Precedence::None);
    result.add_rule(Eof, None, None, Precedence::None);
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_rules() {
    init();
    trace_enter!();
    let _rules = Rules::default();
    trace_exit!();
  }
}
