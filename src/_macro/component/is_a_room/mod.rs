#[macro_export]
macro_rules! is_a_room {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .is_a_room
      .insert($entity, IsARoom)
      .expect("Unable to insert is-a-room for entity!");
  }};
}
