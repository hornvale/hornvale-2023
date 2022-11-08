/// Formats a 3-tuple of u8s in useful ways.
///
/// ```
/// # #[macro_use] extern crate function_name;
/// # #[macro_use] extern crate hornvale;
/// # use hornvale::*;
/// #
/// # #[named]
/// # fn main() {
/// let test = (32, 24, 125);
/// println!("{}", format_3u8!(test));
/// # }
/// ```
#[macro_export]
macro_rules! format_3u8 {
  ($var: expr) => {{
    format!(
      "#{:02X}{:02X}{:02X} ({:#010b}, {:#010b}, {:#010b}) (+/±: ({}, {}, {}))",
      $var.0, $var.1, $var.2, $var.0, $var.1, $var.2, $var.0, $var.1, $var.2
    )
  }};
}
