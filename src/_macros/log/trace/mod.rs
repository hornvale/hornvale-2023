/// Logs a variable and its value, if at RUST_LOG=trace.
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
/// trace_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! trace_var {
  ($var: expr) => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use log::*;
      trace!("{} = {:#?}", stringify!($var), $var);
    }
  }};
}

/// Traces entry into a function, if at RUST_LOG=trace
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
/// trace_enter!();
/// # }
/// ```
#[macro_export]
macro_rules! trace_enter {
  () => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use ::log::*;
      trace!("[ENTER] {} @ line {}", function_name!(), line!());
    }
  }};
}

/// Traces exit from a function, if at RUST_LOG=trace
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
/// trace_exit!();
/// # }
/// ```
#[macro_export]
macro_rules! trace_exit {
  () => {{
    #[allow(unused_imports)]
    #[cfg(debug_assertions)]
    {
      use ::log::*;
      trace!("[EXIT] {} @ line {}", function_name!(), line!());
    }
  }};
}
