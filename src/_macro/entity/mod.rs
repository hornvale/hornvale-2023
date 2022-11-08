#[macro_export]
macro_rules! get_entity {
  ($system_data: expr, $entity_id: expr) => {{
    $system_data.entities.entity($entity_id.0)
  }};
}

#[macro_export]
macro_rules! get_name {
  ($system_data: expr, $entity: expr) => {{
    $system_data.has_name.get($entity).map(|has_name| &has_name.0)
  }};
}

#[macro_export]
macro_rules! set_name {
  ($system_data: expr, $entity: expr, $name: expr) => {{
    $system_data
      .has_name
      .insert($entity, HasName($name.into()))
      .expect("Unable to insert has-name for entity!");
  }};
}

#[macro_export]
macro_rules! get_description {
  ($system_data: expr, $entity: expr) => {{
    $system_data.has_description.get($entity).unwrap_or_default()
  }};
}

#[macro_export]
macro_rules! set_description {
  ($system_data: expr, $entity: expr, $has_description: expr) => {{
    $system_data
      .has_description
      .insert($entity, $has_description)
      .expect("Unable to insert has-description for entity!");
  }};
}

#[macro_export]
macro_rules! get_initial_description {
  ($system_data: expr, $entity: expr) => {{
    $system_data
      .has_description
      .get($entity)
      .map(|has_description| has_description.initial)
  }};
}

#[macro_export]
macro_rules! set_initial_description {
  ($system_data: expr, $entity: expr, $desc: expr) => {{
    let mut has_description = get_description!($system_data, $entity);
    has_description.initial = Some($desc);
    set_description!($system_data, $entity, has_description);
  }};
}

#[macro_export]
macro_rules! get_brief_description {
  ($system_data: expr, $entity: expr) => {{
    $system_data
      .has_description
      .get($entity)
      .map(|has_description| &has_description.brief)
  }};
}

#[macro_export]
macro_rules! set_brief_description {
  ($system_data: expr, $entity: expr, $desc: expr) => {{
    let mut has_description = get_description!($system_data, $entity);
    has_description.brief = Some($desc);
    set_description!($system_data, $entity, has_description);
  }};
}

#[macro_export]
macro_rules! get_current_room {
  ($system_data: expr, $entity: expr) => {{
    $system_data.is_in_room.get($entity).map(|is_in_room| is_in_room.0)
  }};
}

#[macro_export]
macro_rules! set_current_room {
  ($system_data: expr, $entity: expr, $room: expr) => {{
    $system_data
      .is_in_room
      .insert($entity, IsInRoom(RoomId($room.id())))
      .expect("Unable to insert has-description for entity!");
  }};
}
