#[macro_export]
macro_rules! is_an_object {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .is_an_object
      .insert($entity, IsAnObject)
      .expect("Unable to insert is-an-object for entity!");
  }};
}
