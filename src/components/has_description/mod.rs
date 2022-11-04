/// The `HasDescription` type.
#[derive(Clone, Debug, Default)]
pub struct HasDescription {
  /// Shown only on initial viewings.
  pub initial: Option<String>,
  /// A brief description, shown on subsequent views.
  pub brief: String,
}
