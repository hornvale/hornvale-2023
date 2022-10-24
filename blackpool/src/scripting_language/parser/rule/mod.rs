use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::scripting_language::parser::error::Error;
use crate::scripting_language::parser::precedence::Precedence;
use crate::scripting_language::parser::Parser;
use crate::scripting_language::program::Program;

pub type ParseFn = fn(&mut Parser, &mut Program) -> Result<(), Error>;

/// The `Rule` type.
#[derive(Clone, Display)]
#[display(fmt = "prefix: <fn>, infix: <fn>, precedence: {}", precedence)]
pub struct Rule {
  /// Prefix function, for when this token appears as a prefix.
  pub prefix: Option<ParseFn>,
  /// Infix function, for when this token appears as an infix.
  pub infix: Option<ParseFn>,
  /// Precedence.
  pub precedence: Precedence,
}

impl Debug for Rule {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let prefix = match self.prefix {
      Some(_) => "Some(<fn>)",
      None => "None",
    };
    let infix = match self.infix {
      Some(_) => "Some(<fn>)",
      None => "None",
    };
    write!(
      f,
      "Rule {{ prefix: {}, infix: {}, precedence: {} }}",
      prefix, infix, self.precedence
    )
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
    let rule = Rule {
      prefix: None,
      infix: None,
      precedence: Precedence::None,
    };
    trace_var!(rule);
    println!("{:?}", rule);
    trace_exit!();
  }
}
