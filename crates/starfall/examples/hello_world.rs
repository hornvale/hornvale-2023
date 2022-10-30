#![allow(unused_imports)]

use starfall::*;

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  println!("Hello, world!");
  trace_exit!();
}
