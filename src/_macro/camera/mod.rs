#[macro_export]
macro_rules! get_camera_entity_id {
  ($system_data: expr) => {{
    $system_data.camera_resource.0.unwrap()
  }};
}

#[macro_export]
macro_rules! get_camera_entity {
  ($system_data: expr) => {{
    get_entity!($system_data, get_camera_entity_id!($system_data))
  }};
}

#[macro_export]
macro_rules! entity_id_has_camera {
  ($system_data: expr, $entity_id: expr) => {{
    $entity_id == get_camera_entity_id!($system_data)
  }};
}

#[macro_export]
macro_rules! entity_has_camera {
  ($system_data: expr, $entity: expr) => {{
    $entity == get_camera_entity!($system_data)
  }};
}

#[macro_export]
macro_rules! get_camera_room_id {
  ($system_data: expr) => {{
    get_current_room_id!($system_data, get_camera_entity!($system_data))
  }};
}

#[macro_export]
macro_rules! in_camera_room {
  ($system_data: expr, $room: expr) => {{
    use $crate::ecs::entity::RoomId;
    Some(RoomId($room.id())) == get_camera_room_id!($system_data)
  }};
}
