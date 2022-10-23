/// Format a nullary opcode in a useful way.
#[macro_export]
macro_rules! format_nullary_opcode {
  ($opcode: expr) => {{
    format!("{0:<16}", stringify!($opcode))
  }};
}
