#[macro_export]
macro_rules! create_effect {
  ($obj: expr) => {{
    use std::sync::Arc;
    use $crate::effect::Effect;
    Effect(Arc::new($obj))
  }};
}
