#![allow(unused_macros)]

pub use appleton::*;
pub use blackpool::*;
pub use casterlyrock::*;
pub use dreadfort::*;
pub use goldengrove::*;
pub use honeyholt::*;
pub use kingsgrave::*;
pub use lemonwood::*;
pub use ramsgate::*;
pub use starfall::*;
pub use stonedance::*;
pub use volmark::*;
pub use weepingtown::*;

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
