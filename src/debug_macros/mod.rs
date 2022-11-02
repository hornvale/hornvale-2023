/// Logs a variable and its value, if at RUST_LOG=trace.
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
///
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

/// Logs a variable and its value, if at RUST_LOG>=debug
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

/// Logs a variable and its value, if at RUST_LOG>=info
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

/// Traces entry into a function, if at RUST_LOG=trace
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate function_name;
/// # #[macro_use] extern crate hornvale;
/// # use hornvale::*;
/// #
/// # fn main() {
///
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
/// # fn main() {
///
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

/// Prints a u8 variable, if at RUST_LOG=trace
#[macro_export]
macro_rules! trace_u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    trace!("{} = {}", stringify!($var), format_u8!($var));
  };
}

/// Prints a u8 variable, if at RUST_LOG=debug
#[macro_export]
macro_rules! debug_u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    debug!("{} = {}", stringify!($var), format_u8!($var));
  };
}

/// Prints a u8 variable, if at RUST_LOG=info
#[macro_export]
macro_rules! info_u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    info!("{} = {}", stringify!($var), format_u8!($var));
  };
}

/// Formats a u8 variable in a useful way.
#[macro_export]
macro_rules! format_u8 {
  ($var: expr) => {{
    if $var & 0x80 > 0 {
      format!("{:#04X} {:#010b} (+: {}, ±: {})", $var, $var, $var as u8, $var as i8)
    } else {
      format!("{:#04X} {:#010b} (+/±: {})", $var, $var, $var as u8)
    }
  }};
}

/// Prints a 3-tuple of u8s, if at RUST_LOG=trace
#[macro_export]
macro_rules! trace_3u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    trace!("{} = {}", stringify!($var), format_3u8!($var));
  };
}

/// Prints a 3-tuple of u8s, if at RUST_LOG=debug
#[macro_export]
macro_rules! debug_3u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    debug!("{} = {}", stringify!($var), format_3u8!($var));
  };
}

/// Prints a 3-tuple of u8s, if at RUST_LOG=info
#[macro_export]
macro_rules! info_3u8 {
  ($var: expr) => {
    #[cfg(debug_assertions)]
    info!("{} = {}", stringify!($var), format_3u8!($var));
  };
}

/// Formats a 3-tuple of u8s in useful ways.
#[macro_export]
macro_rules! format_3u8 {
  ($var: expr) => {{
    format!(
      "#{:02X}{:02X}{:02X} ({:#010b}, {:#010b}, {:#010b}) (+/±: ({}, {}, {}))",
      $var.0, $var.1, $var.2, $var.0, $var.1, $var.2, $var.0, $var.1, $var.2
    )
  }};
}
