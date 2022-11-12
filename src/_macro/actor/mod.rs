#[macro_export]
macro_rules! create_actor {
  ($data: expr, $name: expr, $description: expr) => {{
    let actor = $data.entities.create();
    is_an_actor!($data, actor);
    has_name!($data, actor, $name);
    has_initiative!($data, actor, 1);
    has_brief_description!($data, actor, $description);
    actor
  }};
  ($data: expr, $name: expr, $description: expr, $room_id: expr) => {{
    let actor = create_actor!($data, $name, $description);
    is_in_room!($data, actor, $room_id);
    actor
  }};
}
