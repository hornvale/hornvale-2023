#[macro_export]
macro_rules! create_object {
  ($data: expr, $name: expr, $brief_description: expr) => {{
    let object = $data.entities.create();
    has_name!($data, object, $name);
    has_brief_description!($data, object, $brief_description);
    is_an_object!($data, object);
    object
  }};
  ($data: expr, $name: expr, $brief_description: expr, $in_room: expr) => {{
    let object = create_object!($data, $name, $brief_description);
    is_in_room!($data, object, $in_room);
    object
  }};
}
