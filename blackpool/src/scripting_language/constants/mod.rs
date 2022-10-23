use crate::scripting_language::value::Value;

pub type ConstantsIndexType = u8;

/// The `Constants` type.
///
/// This represents a constant pool.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Constants {
  pub values: Vec<Value>,
}

impl Constants {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let values = Vec::new();
    let result = Constants { values };
    trace_var!(result);
    trace_exit!();
    result
  }
}
