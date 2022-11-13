#[macro_export]
macro_rules! you {
  ($data: expr, $entity: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if entity_has_camera!($data, $entity) {
      write_output_event!($data, format!("You {}", $string));
    }
  }};
}
