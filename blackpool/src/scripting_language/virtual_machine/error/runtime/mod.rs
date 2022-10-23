/// Errors encountered at runtime.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum RuntimeError {
  /// Stack overflow.
  #[error("stack overflow")]
  StackOverflow,
  /// Stack underflow.
  #[error("stack underflow")]
  StackUnderflow,
}
