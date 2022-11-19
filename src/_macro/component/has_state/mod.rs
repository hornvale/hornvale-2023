#[macro_export]
macro_rules! has_state {
  ($data: expr, $entity: expr) => {{
    #[allow(unused_imports)]
    use $crate::component::*;
    use $crate::goap::State;
    $data
      .has_state
      .insert($entity, HasState(State::default()))
      .expect("Unable to insert has-state for entity!");
  }};
}

#[macro_export]
macro_rules! get_state {
  ($data: expr, $entity: expr) => {{
    $data.has_state.get($entity).map(|has_state| &has_state.0)
  }};
}

#[macro_export]
macro_rules! set_state {
  ($data: expr, $entity: expr, $position: expr) => {{
    if let Some(has_state) = $data.has_state.get_mut($entity) {
      has_state.0.values |= 1 << $position;
    }
  }};
}

#[macro_export]
macro_rules! reset_state {
  ($data: expr, $entity: expr, $position: expr) => {{
    if let Some(has_state) = $data.has_state.get_mut($entity) {
      has_state.0.values &= !(1 << $position);
    }
  }};
}
