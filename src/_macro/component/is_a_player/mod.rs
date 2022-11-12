#[macro_export]
macro_rules! is_a_player {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::components::*;
    $data
      .is_a_player
      .insert($entity, IsAPlayer)
      .expect("Unable to insert is_a_player for entity!");
  }};
}
