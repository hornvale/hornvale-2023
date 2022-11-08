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
macro_rules! get_initial_description {
  ($system_data: expr, $entity: expr) => {{
    $system_data
      .has_description
      .get($entity)
      .map(|has_description| has_description.initial)
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
macro_rules! get_current_room {
  ($system_data: expr, $entity: expr) => {{
    $system_data.is_in_room.get($entity).map(|is_in_room| is_in_room.0)
  }};
}
