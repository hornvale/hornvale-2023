/// Logs a variable and its value, if at RUST_LOG>=debug
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
/// debug_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! debug_var {
  ($var: expr) => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use ::log::*;
      debug!("{} = {:#?}", stringify!($var), $var);
    }
  }};
}
