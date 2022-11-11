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

  pub fn new(enclosing: Option<Box<ClassCompiler>>) -> Box<Self> {
    let has_superclass = false;
    let result = ClassCompiler {
      enclosing,
      has_superclass,
    };

    Box::new(result)
  }
}
