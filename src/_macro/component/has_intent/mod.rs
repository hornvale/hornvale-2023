#[macro_export]
macro_rules! has_intent {
  ($data: expr, $entity: expr, $action: expr, $priority: expr, $initiative_cost: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::components::*;
    use $crate::intent::Intent;
    use $crate::priority::Priority;
    $data
      .has_intent
      .insert(
        $entity,
        HasIntent(Intent {
          action: $action,
          priority: $priority,
          initiative_cost: $initiative_cost,
        }),
      )
      .expect("Unable to insert has-intent for entity!");
  }};
}
