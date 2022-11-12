#[macro_export]
macro_rules! is_in_room {
  ($data: expr, $entity: expr, $room_id: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::components::*;
    $data
      .is_in_room
      .insert($entity, IsInRoom($room_id))
      .expect("Unable to insert is_in_room for entity!");
  }};
}
