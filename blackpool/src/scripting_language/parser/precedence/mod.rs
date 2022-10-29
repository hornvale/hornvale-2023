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
  pub fn next(&self) -> Precedence {
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
      Primary => None,
    };
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
  pub fn test_precedence() {
    init();
    trace_enter!();
    let mut precedence = Precedence::Assignment;
    while precedence != Precedence::None {
      precedence = precedence.next();
    }
    trace_exit!();
  }
}
