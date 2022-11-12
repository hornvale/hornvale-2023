#[macro_export]
macro_rules! create_player {
  ($data: expr, $gender: expr) => {{
    let player = $data.entities.create();
    is_an_actor!($data, player);
    is_a_player!($data, player);
    has_name!($data, player, "Player");
    has_initiative!($data, player, 1);
    has_brief_description!($data, player, "It's you, you idiot!");
    has_gender!($data, player, $gender);
    player
  }};
  ($data: expr, $in_room: expr, $gender: expr) => {{
    use $crate::component::*;
    let player = create_player!($data);
    is_in_room!($data, player, $room_id, $gender);
    player
  }};
}
