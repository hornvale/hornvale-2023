use rustyline_async::SharedWriter;

/// The `Output` resource.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Output(pub Option<SharedWriter>);
