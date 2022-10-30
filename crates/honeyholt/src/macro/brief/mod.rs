/// Implements a human-friendly brief display for a type.
#[macro_export]
macro_rules! honeyholt_define_brief {
  ($struct_name: ident, $impl: expr) => {
    use honeyholt::r#trait::brief::Brief;
    use std::fmt;
    impl Brief for $struct_name {
      fn honeyholt_display_brief<'a>(&'a self) -> Box<dyn fmt::Display + 'a> {
        struct MyDisplay<'a>(pub &'a $struct_name);
        impl<'a> fmt::Display for MyDisplay<'a> {
          fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", ($impl)(self.0))
          }
        }
        Box::new(MyDisplay(self))
      }
    }
  };
}

/// Retrieves a human-friendly brief description.
#[macro_export]
macro_rules! honeyholt_brief {
  ($var: expr) => {{
    use honeyholt::r#trait::brief::Brief;
    format!("{}", $var.honeyholt_display_brief())
  }};
}
