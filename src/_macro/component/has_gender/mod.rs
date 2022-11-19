#[macro_export]
macro_rules! has_gender {
  ($data: expr, $entity: expr, $gender: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::component::*;
    use $crate::gender::Gender;
    $data
      .has_gender
      .insert($entity, HasGender($gender))
      .expect("Unable to insert has-gender for entity!");
  }};
}

#[macro_export]
macro_rules! get_gender {
  ($data: expr, $entity: expr) => {{
    $data.has_gender.get($entity).map(|has_gender| &has_gender.0)
  }};
}
