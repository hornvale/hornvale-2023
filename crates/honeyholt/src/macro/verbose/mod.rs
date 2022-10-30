/// Implements a human-friendly verbose display for a type.
#[macro_export]
macro_rules! honeyholt_define_verbose {
  ($struct_name: ident, $impl: expr) => {
    use honeyholt::r#trait::verbose::Verbose;
    use std::fmt;
    impl Verbose for $struct_name {
      fn honeyholt_display_verbose<'a>(&'a self) -> Box<dyn fmt::Display + 'a> {
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

/// Retrieves a human-friendly verbose description.
#[macro_export]
macro_rules! honeyholt_verbose {
  ($var: expr) => {{
    use honeyholt::r#trait::verbose::Verbose;
    format!("{}", $var.honeyholt_display_verbose())
  }};
}
