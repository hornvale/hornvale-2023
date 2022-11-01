use crate::parsing::error::Error;
use crate::parsing::parser::Parser;

/// The `TwoWord` type.
///
/// This will actually be slightly more than a two-word parser, but who cares?
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq)]
pub struct TwoWord {}

impl Parser for TwoWord {
  /// Parse two (or more) words of input.
  #[named]
  fn parse_input(&mut self, input: &str) -> Result<Option<String>, Error> {
    trace_enter!();
    let words = input.split(' ').map(str::to_string).collect::<Vec<String>>();
    trace_var!(words);
    let result = match words[0].as_str() {
      "look" => Some("You see a lot of WTF.".to_owned()),
      "west" | "w" => Some("You can't go west right now (you're not that smart).".to_owned()),
      &_ => todo!(),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
