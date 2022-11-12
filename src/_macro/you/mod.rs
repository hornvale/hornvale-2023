#[macro_export]
macro_rules! you {
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
