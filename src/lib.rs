#![allow(unused_macros)]

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;
extern crate cpu_time;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
pub use log::*;
extern crate pretty_env_logger;
pub use pretty_env_logger::env_logger::builder as pretty_env_logger_builder;
pub use pretty_env_logger::init as init_pretty_env_logger;
#[macro_use]
extern crate serde;
extern crate specs;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate thiserror;

// Utilities shared and relied upon by all systems.
#[macro_use]
pub mod _macro;
pub use _macro::*;
pub mod _error;

// Systems.
pub mod action;
pub mod anatomy;
pub mod astronomy;
pub mod biology;
pub mod combat;
pub mod command;
pub mod downdelving;
pub mod economics;
pub mod ecs;
pub mod effect;
pub mod formatting;
pub mod game;
pub mod geology;
pub mod goap;
pub mod linguistics;
pub mod map;
pub mod mythopoetics;
pub mod perception;
pub mod scripting_language;
pub mod sociology;
pub mod supernatural;

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
