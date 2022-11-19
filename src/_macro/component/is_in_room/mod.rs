#[macro_export]
macro_rules! is_in_room {
  ($data: expr, $entity: expr, $room_id: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::component::*;
    $data
      .is_in_room
      .insert($entity, IsInRoom($room_id))
      .expect("Unable to insert is_in_room for entity!");
  }};
}

#[macro_export]
macro_rules! remove_is_in_room {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::component::*;
    $data
      .is_in_room
      .remove($entity)
      .expect("Unable to remove is_in_room for entity!");
  }};
}

#[macro_export]
macro_rules! get_current_room_id {
  ($data: expr, $entity: expr) => {{
    $data.is_in_room.get($entity).map(|is_in_room| is_in_room.0)
  }};
}
