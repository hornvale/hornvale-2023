#[macro_export]
macro_rules! has_initiative {
  ($data: expr, $entity: expr, $current: expr, $increment: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    use $crate::initiative::Initiative;
    $data
      .has_initiative
      .insert(
        $entity,
        HasInitiative(Initiative {
          current: $current,
          increment: $increment,
        }),
      )
      .expect("Unable to insert has-initiative for entity!");
  }};
}

#[macro_export]
macro_rules! get_has_initiative {
  ($data: expr, $entity: expr) => {{
    $data
      .has_initiative
      .get($entity)
      .map(|has_initiative| &has_initiative.0)
  }};
}
