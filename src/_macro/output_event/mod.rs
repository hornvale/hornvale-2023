#[macro_export]
macro_rules! get_output_event_channel {
  ($data: expr) => {{
    &mut $data.output_event_channel
  }};
}

#[macro_export]
macro_rules! write_output_event {
  ($data: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    get_output_event_channel!($data).single_write(OutputEvent { string: $string.into() });
  }};
}

#[macro_export]
macro_rules! write_output_3rd {
  ($data: expr, $entity: expr, $room: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if in_camera_room!($data, $room) && !entity_has_camera!($data, $entity) {
      write_output_event!($data, $string);
    }
  }};
}
