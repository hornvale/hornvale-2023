/// This is for presenting the player (or camera) with information.
///
/// This should not take into consideration things like blindness, etc.
/// This is more for error messages, action descriptions, etc.  Those may take
/// those things into account, but we will not filter them further.
///
/// See:
/// - you_see!() for visual information
/// - you_hear!() for auditory information
/// - you_smell!() for olfactory information
/// - you_feel!() for touch/intuitive information
/// - you_taste!() for taste information
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
