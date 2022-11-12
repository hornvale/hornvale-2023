#[macro_export]
macro_rules! get_entity {
  ($data: expr, $entity_id: expr) => {{
    $data.entities.entity($entity_id.0)
  }};
}

#[macro_export]
macro_rules! get_initial_description {
  ($data: expr, $entity: expr) => {{
    $data
      .has_brief_description
      .get($entity)
      .map(|has_brief_description| has_brief_description.initial)
  }};
}

#[macro_export]
macro_rules! set_initial_description {
  ($data: expr, $entity: expr, $desc: expr) => {{
    let mut has_brief_description = get_description!($data, $entity);
    has_brief_description.initial = Some($desc);
    set_description!($data, $entity, has_brief_description);
  }};
}

#[macro_export]
macro_rules! get_current_room_id {
  ($data: expr, $entity: expr) => {{
    $data.is_in_room.get($entity).map(|is_in_room| is_in_room.0)
  }};
}

#[macro_export]
macro_rules! set_current_room_id {
  ($data: expr, $entity: expr, $room: expr) => {{
    use $crate::ecs::components::IsInRoom;
    use $crate::ecs::entity::RoomId;
    $data
      .is_in_room
      .insert($entity, IsInRoom(RoomId($room.id())))
      .expect("Unable to insert has-description for entity!");
  }};
}
