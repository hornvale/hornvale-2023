#[macro_export]
macro_rules! clone_output {
  ($system_data: expr) => {{
    #[allow(unused_imports)]
    use std::io::Write as _;
    let output_resource = $system_data.output_resource.0.as_ref().unwrap();
    output_resource.clone()
  }};
}

#[macro_export]
macro_rules! get_output {
  ($system_data: expr) => {{
    #[allow(unused_imports)]
    &mut $system_data.output_event_channel
  }};
}

#[macro_export]
macro_rules! write_output {
  ($system_data: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::event_channels::OutputEvent;
    get_output!($system_data).single_write(OutputEvent { string: $string });
  }};
}

#[macro_export]
macro_rules! write_output_2nd {
  ($system_data: expr, $entity: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::event_channels::OutputEvent;
    if entity_has_camera!($system_data, $entity) {
      get_output!($system_data).single_write(OutputEvent {
        string: format!("{}", $string),
      });
    }
  }};
}

#[macro_export]
macro_rules! write_output_3rd {
  ($system_data: expr, $entity: expr, $room: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::event_channels::OutputEvent;
    if in_camera_room!($system_data, $room) && !entity_has_camera!($system_data, $entity) {
      get_output!($system_data).single_write(OutputEvent { string: $string.into() });
    }
  }};
}
