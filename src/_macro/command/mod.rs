#[macro_export]
macro_rules! create_command {
  ($obj: expr) => {{
    use std::sync::Arc;
    use $crate::command::Command;
    Command(Arc::new($obj))
  }};
}
