#[macro_export]
macro_rules! get_name {
  ($system_data: expr, $entity: expr) => {{
    let mut result = None;
    if let Some(HasName(name)) = $system_data.has_name.get($entity) {
      result = Some(name);
    }
    result
  }};
}

#[macro_export]
macro_rules! get_description {
  ($system_data: expr, $entity: expr) => {{
    let mut result = None;
    if let Some(HasDescription { brief, .. }) = $system_data.has_description.get($entity) {
      result = Some(brief);
    }
    result
  }};
}

#[macro_export]
macro_rules! get_current_room {
  ($system_data: expr, $entity: expr) => {{
    let mut result = None;
    if let Some(is_in_room) = $system_data.is_in_room.get($entity) {
      result = Some(is_in_room.0);
    }
    result
  }};
}
