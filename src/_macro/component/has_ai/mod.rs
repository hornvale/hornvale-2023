#[macro_export]
macro_rules! has_ai {
  ($data: expr, $entity: expr, $ai: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .has_ai
      .insert($entity, HasAi($ai))
      .expect("Unable to insert has-ai for entity!");
  }};
}
