#[macro_export]
macro_rules! get_action_event_channel {
  ($data: expr) => {{
    &mut $data.action_event_channel
  }};
}

#[macro_export]
macro_rules! write_action_event {
  ($data: expr, $action: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::event::ActionEvent;
    get_action_event_channel!($data).single_write(ActionEvent { action: $action });
  }};
}
