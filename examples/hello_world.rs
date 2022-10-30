#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use hornvale::*;

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  println!("Hello, world!");
  trace_exit!();
}
