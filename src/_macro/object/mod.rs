#[macro_export]
macro_rules! create_object {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use $crate::ecs::components::*;
    let object_id = $system_data.entities.create();
    $system_data
      .has_name
      .insert(object_id, HasName($name.into()))
      .expect("Unable to insert name for entity!");
    $system_data
      .has_description
      .insert(
        object_id,
        HasDescription {
          initial: None,
          brief: $description.into(),
        },
      )
      .expect("Unable to insert description for entity!");
    $system_data
      .is_an_object
      .insert(object_id, IsAnObject)
      .expect("Unable to insert is-an-object for entity!");
    object_id
  }};
  ($system_data: expr, $name: expr, $description: expr, $in_room: expr) => {{
    use $crate::ecs::components::*;
    let object_id = $system_data.entities.create();
    $system_data
      .has_name
      .insert(object_id, HasName($name.into()))
      .expect("Unable to insert name for entity!");
    $system_data
      .has_description
      .insert(
        object_id,
        HasDescription {
          initial: None,
          brief: $description.into(),
        },
      )
      .expect("Unable to insert description for entity!");
    $system_data
      .is_an_object
      .insert(object_id, IsAnObject)
      .expect("Unable to insert is-an-object for entity!");
    $system_data
      .is_in_room
      .insert(object_id, IsInRoom($in_room))
      .expect("Unable to insert is-in-room for entity!");
    object_id
  }};
}
