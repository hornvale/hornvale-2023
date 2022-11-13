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
  pub fn next(&self) -> Precedence {
    use Precedence::*;
    match self {
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
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_precedence() {
    init();
    let mut precedence = Precedence::Assignment;
    while precedence != Precedence::None {
      precedence = precedence.next();
    }
  }
}
