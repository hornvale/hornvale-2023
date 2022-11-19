/// This is for presenting the player (or camera) with information.
///
/// This should take into consideration things like blindness, etc.
#[macro_export]
macro_rules! you_see {
  ($data: expr, $entity: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if entity_has_camera!($data, $entity) {
      write_output_event!($data, format!("You see {}", $string));
    }
  }};
}
