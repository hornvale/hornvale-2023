#[macro_export]
macro_rules! has_passages {
  ($data: expr, $room: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .has_passages
      .insert($room, HasPassages::default())
      .expect("Unable to insert has-passages for entity!");
  }};
}
