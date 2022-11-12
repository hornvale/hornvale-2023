#[macro_export]
macro_rules! clone_output {
  ($data: expr) => {{
    #[allow(unused_imports)]
    use std::io::Write as _;
    let output_resource = $data.output_resource.0.as_ref().unwrap();
    output_resource.clone()
  }};
}

#[macro_export]
macro_rules! get_output {
  ($data: expr) => {{
    #[allow(unused_imports)]
    &mut $data.output_event_channel
  }};
}

#[macro_export]
macro_rules! write_output {
  ($data: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    get_output!($data).single_write(OutputEvent { string: $string });
  }};
}

#[macro_export]
macro_rules! write_output_2nd {
  ($data: expr, $entity: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if entity_has_camera!($data, $entity) {
      get_output!($data).single_write(OutputEvent {
        string: format!("{}", $string),
      });
    }
  }};
}

#[macro_export]
macro_rules! write_output_3rd {
  ($data: expr, $entity: expr, $room: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if in_camera_room!($data, $room) && !entity_has_camera!($data, $entity) {
      get_output!($data).single_write(OutputEvent { string: $string.into() });
    }
  }};
}
