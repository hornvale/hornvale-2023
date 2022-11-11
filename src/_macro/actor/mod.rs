#[macro_export]
macro_rules! create_actor {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use $crate::ecs::components::*;
    let actor = $system_data.entities.create();
    $system_data
      .is_an_actor
      .insert(actor, IsAnActor)
      .expect("Unable to insert is-an-actor for entity!");
    $system_data
      .has_name
      .insert(actor, HasName($name.into()))
      .expect("Unable to insert name for entity!");
    $system_data
      .has_description
      .insert(
        actor,
        HasDescription {
          initial: None,
          brief: $description.into(),
        },
      )
      .expect("Unable to insert description for entity!");
    actor
  }};
  ($system_data: expr, $name: expr, $description: expr, $in_room: expr) => {{
    use $crate::ecs::components::*;
    let actor = create_actor!($system_data, $name, $description);
    $system_data
      .is_in_room
      .insert(actor, IsInRoom($in_room))
      .expect("Unable to insert is-in-room for entity!");
    actor
  }};
}
