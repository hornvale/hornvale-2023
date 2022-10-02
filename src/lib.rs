#![allow(unused_macros)]

pub use breakwater::*;
pub use brownhollow::*;
pub use goldengrove::*;
pub use lasthearth::*;
pub use volmark::*;

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
