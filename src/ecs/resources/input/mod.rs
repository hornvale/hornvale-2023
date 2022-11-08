use rustyline_async::Readline;

/// The `Input` resource.
#[derive(Default)]
pub struct Input(pub Option<Readline>);
