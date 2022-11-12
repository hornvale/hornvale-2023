#[macro_export]
macro_rules! get_output {
  ($data: expr) => {{
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
macro_rules! write_output_3rd {
  ($data: expr, $entity: expr, $room: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if in_camera_room!($data, $room) && !entity_has_camera!($data, $entity) {
      get_output!($data).single_write(OutputEvent { string: $string.into() });
    }
  }};
}
