/// Logs a variable and its value, if at RUST_LOG>=info
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate function_name;
/// # #[macro_use] extern crate hornvale;
/// # use hornvale::*;
/// #
/// # #[named]
/// # fn main() {
/// let five = 5;
/// info_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! info_var {
  ($var: expr) => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use ::log::*;
      info!("{} = {:#?}", stringify!($var), $var);
    }
  }};
}
