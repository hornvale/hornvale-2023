#[macro_export]
macro_rules! they {
  ($data: expr, $entity: expr, $string: expr) => {{
    if camera_sees_entity!($data, $entity) {
      write_output_event!($data, $string);
    }
  }};
}
