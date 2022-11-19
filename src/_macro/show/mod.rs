/// This macro is for showing the player (or camera) things.
///
/// When we have conditions like blindness, etc, we will have to take that into
/// account.
///
/// This should not be confused with `you_see!()`, which is used more for brief
/// presentations, like "you see a cloud cross in front of the sun".  This is
/// visual information presented without preface.
#[macro_export]
macro_rules! show {
  ($data: expr, $entity: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::OutputEvent;
    if entity_has_camera!($data, $entity) {
      write_output_event!($data, $string);
    }
  }};
}
