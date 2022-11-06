/// Logs a variable and its value unconditionally
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate function_name;
/// # #[macro_use] extern crate hornvale;
/// # use hornvale::*;
/// #
/// # fn main() {
/// let five = 5;
/// print_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! print_var {
  ($var: expr) => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use ::log::*;
      println!("{} = {:#?}", stringify!($var), $var);
    }
  }};
}
