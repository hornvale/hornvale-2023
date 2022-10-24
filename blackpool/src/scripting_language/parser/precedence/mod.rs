/// The `Precedence` enum.
///
/// This allows controlling the parser flow.
#[derive(Clone, Copy, Debug, Display, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Precedence {
  None,
  Assignment, // =
  Or,         // ||
  And,        // &&
  Equality,   // == !=
  Comparison, // < > <= >=
  Term,       // + -
  Factor,     // * /
  Unary,      // ! -
  Call,       // . ()
  Primary,
}

impl Precedence {
  #[named]
  pub fn next(&self) -> Option<Precedence> {
    trace_enter!();
    use Precedence::*;
    let result = match self {
      None => Assignment,
      Assignment => Or,
      Or => And,
      And => Equality,
      Equality => Comparison,
      Comparison => Term,
      Term => Factor,
      Factor => Unary,
      Unary => Call,
      Call => Primary,
      Primary => return Option::None,
    };
    trace_var!(result);
    trace_exit!();
    Some(result)
  }
}
