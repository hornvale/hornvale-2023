#[macro_export]
macro_rules! create_action {
  ($obj: expr) => {{
    use std::sync::Arc;
    use $crate::action::Action;
    Action(Arc::new($obj))
  }};
}

#[macro_export]
macro_rules! action_error {
  ($data: expr, $action: expr, $error: expr) => {{
    use inflector::Inflector;
    show!($data, get_entity!($data, $action.get_actor_entity_id()), {
      format!("{}.", format!("{}", $error).to_sentence_case())
    });
  }};
}
