#[macro_export]
macro_rules! get_camera_entity_id {
  ($data: expr) => {{
    $data.camera_resource.0.unwrap()
  }};
}

#[macro_export]
macro_rules! get_camera_entity {
  ($data: expr) => {{
    get_entity!($data, get_camera_entity_id!($data))
  }};
}

#[macro_export]
macro_rules! entity_id_has_camera {
  ($data: expr, $entity_id: expr) => {{
    $entity_id == get_camera_entity_id!($data)
  }};
}

#[macro_export]
macro_rules! entity_has_camera {
  ($data: expr, $entity: expr) => {{
    $entity == get_camera_entity!($data)
  }};
}

#[macro_export]
macro_rules! get_camera_room_id {
  ($data: expr) => {{
    get_current_room_id!($data, get_camera_entity!($data))
  }};
}

#[macro_export]
macro_rules! in_camera_room {
  ($data: expr, $room: expr) => {{
    use $crate::ecs::entity::RoomId;
    Some(RoomId($room.id())) == get_camera_room_id!($data)
  }};
}
