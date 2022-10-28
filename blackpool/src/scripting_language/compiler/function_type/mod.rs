/// The `FunctionType` enum.
///
/// This is the type of function that this compiler is compiling.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum FunctionType {
  /// A function!
  Function,
  /// A script!
  Script,
}
