use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use crate::scripting_language::token::error::Error as TokenError;

/// The `Type` enum.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Type {
  LeftParenthesis,
  RightParenthesis,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
  Identifier,
  String,
  Number,
  And,
  Class,
  Else,
  False,
  Function,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
  ScannerError(&'static str),
  EmptyString,
}

impl Type {
  #[named]
  pub fn get_all() -> Vec<Self> {
    use Type::*;
    vec![
      LeftParenthesis,
      RightParenthesis,
      LeftBrace,
      RightBrace,
      Comma,
      Dot,
      Minus,
      Plus,
      Semicolon,
      Slash,
      Star,
      Bang,
      BangEqual,
      Equal,
      EqualEqual,
      GreaterThan,
      GreaterThanOrEqual,
      LessThan,
      LessThanOrEqual,
      Identifier,
      String,
      Number,
      And,
      Class,
      Else,
      False,
      Function,
      For,
      If,
      Nil,
      Or,
      Print,
      Return,
      Super,
      This,
      True,
      Var,
      While,
      Eof,
      ScannerError(""),
      EmptyString,
    ]
  }
}

impl Display for Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:?}", self)
  }
}

impl FromStr for Type {
  type Err = TokenError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    use Type::*;
    match string {
      "and" | "&&" => Ok(And),
      "class" => Ok(Class),
      "else" => Ok(Else),
      "false" => Ok(False),
      "fun" | "fn" | "function" => Ok(Function),
      "for" => Ok(For),
      "if" => Ok(If),
      "nil" => Ok(Nil),
      "or" | "||" => Ok(Or),
      "print" => Ok(Print),
      "return" => Ok(Return),
      "super" => Ok(Super),
      "this" => Ok(This),
      "true" => Ok(True),
      "var" | "let" => Ok(Var),
      "while" => Ok(While),
      unknown => Err(TokenError::UnknownKeyword(unknown.to_string())),
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test() {
    init();
    trace_enter!();
    let all = Type::get_all();
    print_var!(all);
    println!("{}", all.last().unwrap());
    trace_exit!();
  }
}
