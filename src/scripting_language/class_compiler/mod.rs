/// The `ClassCompiler` type.
#[derive(Clone, Debug)]
pub struct ClassCompiler {
  /// The class compiler enclosing this one.
  pub enclosing: Option<Box<ClassCompiler>>,
  /// Whether the compiled class has a superclass.
  pub has_superclass: bool,
}

impl ClassCompiler {
  /// Constructor.
  #[named]
  pub fn new(enclosing: Option<Box<ClassCompiler>>) -> Box<Self> {
    trace_enter!();
    trace_var!(enclosing);
    let has_superclass = false;
    trace_var!(has_superclass);
    let result = ClassCompiler {
      enclosing,
      has_superclass,
    };
    trace_var!(result);
    trace_exit!();
    Box::new(result)
  }
}
