#[macro_export]
macro_rules! create_action {
  ($obj: expr) => {{
    use std::sync::Arc;
    use $crate::action::Action;
    Action(Arc::new($obj))
  }};
}
