#[macro_export]
macro_rules! create_being {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use $crate::ecs::components::*;
    let being = $system_data.entities.create();
    $system_data
      .is_a_being
      .insert(being, IsABeing)
      .expect("Unable to insert is-a-being for entity!");
    $system_data
      .has_name
      .insert(being, HasName($name.into()))
      .expect("Unable to insert name for entity!");
    $system_data
      .has_description
      .insert(
        being,
        HasDescription {
          initial: None,
          brief: $description.into(),
        },
      )
      .expect("Unable to insert description for entity!");
    being
  }};
  ($system_data: expr, $name: expr, $description: expr, $in_room: expr) => {{
    use $crate::ecs::components::*;
    let being = create_being!($system_data, $name, $description);
    $system_data
      .is_in_room
      .insert(being, IsInRoom($in_room))
      .expect("Unable to insert is-in-room for entity!");
    being
  }};
}
