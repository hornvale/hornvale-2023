#[macro_export]
macro_rules! is_an_actor {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .is_an_actor
      .insert($entity, IsAnActor)
      .expect("Unable to insert is-an-actor for entity!");
  }};
}
