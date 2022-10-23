use crate::scripting_language::program::Program;
use crate::scripting_language::scanner::Scanner;
use crate::scripting_language::token::r#type::Type as TokenType;

pub mod error;
use error::Error;

/// The `Compiler` type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Display, Hash, PartialEq, Serialize)]
pub struct Compiler {}

impl Compiler {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let result = Self {};
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Constructor.
  #[named]
  pub fn compile(&self, source: &String) -> Result<Program, Error> {
    trace_enter!();
    trace_var!(source);
    let mut scanner = Scanner::new(source);
    let line_number: usize = usize::MAX;
    loop {
      let token = scanner.scan_token()?;
      if token.r#type == TokenType::Eof {
        break;
      }
      if token.line_number != line_number {
        print!("{:>4} ", token.line_number);
      } else {
        print!("   | ");
      }
      println!("{} {} {}", token.r#type, token.length, token.start);
    }
    let result = Program::default();
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_scanner() {
    init();
    trace_enter!();
    let compiler = Compiler::default();
    print_var!(compiler);
    trace_exit!();
  }
}
