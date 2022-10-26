/// The `Compiler` type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Display, Hash, PartialEq, Serialize)]
pub struct Compiler {}

impl Compiler {}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_compiler() {
    init();
    trace_enter!();
    let compiler = Compiler::default();
    print_var!(compiler);
    trace_exit!();
  }
}
