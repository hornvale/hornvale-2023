#[macro_export]
macro_rules! has_brief_description {
  ($data: expr, $entity: expr, $description: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    $data
      .has_brief_description
      .insert($entity, HasBriefDescription($description.into()))
      .expect("Unable to insert has-description for entity!");
  }};
}

#[macro_export]
macro_rules! get_brief_description {
  ($data: expr, $entity: expr) => {{
    $data.has_brief_description.get($entity)
  }};
}
