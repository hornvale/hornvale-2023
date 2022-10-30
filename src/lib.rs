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
extern crate function_name;
#[macro_use]
extern crate log;
pub use log::*;
extern crate pretty_env_logger;
pub use pretty_env_logger::env_logger::builder as pretty_env_logger_builder;
pub use pretty_env_logger::init as init_pretty_env_logger;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

#[macro_use]
pub mod debug_macros;
pub use debug_macros::*;

/// Astronomy and Astrophysics
pub mod astronomy;
// Goal-Oriented Action Planning
pub mod goap;
// Embedded Scripting Language
pub mod scripting_language;

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  #[named]
  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
