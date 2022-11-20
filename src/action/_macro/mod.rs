#[macro_export]
macro_rules! on_variants {
  ( $self: expr, $type: ident, $method: ident $( , $arg: ident )* ) => {{
    use $type::*;
    match &$self {
      GoDirection(action) => action.$method($($arg),*),
      Idle(action) => action.$method($($arg),*),
      LookAround(action) => action.$method($($arg),*),
      LookAtEntity(action) => action.$method($($arg),*),
      LookDirection(action) => action.$method($($arg),*),
    }
  }};
}
