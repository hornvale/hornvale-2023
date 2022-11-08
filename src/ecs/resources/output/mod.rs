use rustyline_async::SharedWriter;

/// The `Output` resource.
#[derive(Clone, Default)]
pub struct Output(pub Option<SharedWriter>);
