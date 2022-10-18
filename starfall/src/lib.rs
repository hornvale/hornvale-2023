#![allow(unused_macros)]

#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;

#[macro_use]
extern crate honeyholt;
pub use honeyholt::*;

#[macro_use]
extern crate volmark;
pub use volmark::*;

pub mod astronomy;

#[cfg(test)]
pub mod test {

  use std::env::set_var;

  pub use super::*;

  #[named]
  pub fn init() {
    let _ = pretty_env_logger_builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
