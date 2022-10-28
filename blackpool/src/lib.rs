#![allow(unused_macros)]

#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;
extern crate cpu_time;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate volmark;
pub use volmark::*;

pub mod scripting_language;

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  #[named]
  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
