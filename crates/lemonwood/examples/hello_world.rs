#![allow(unused_imports)]
#![allow(unused_macros)]

use lemonwood::*;

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  println!("Hello, world!");
  trace_exit!();
}
