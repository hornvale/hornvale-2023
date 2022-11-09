use rustyline_async::Readline;

/// The `Input` resource.
#[derive(Default)]
#[repr(transparent)]
pub struct Input(pub Option<Readline>);
