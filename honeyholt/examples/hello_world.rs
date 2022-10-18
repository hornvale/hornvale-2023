#![allow(unused_imports)]
#![allow(unused_macros)]

use honeyholt::*;

struct Foo {
  pub foo: i32,
}

honeyholt_define_brief!(Foo, |var: &Foo| { var.foo.to_string() });

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  println!("Hello, world!");
  println!("{}", honeyholt_brief!(Foo { foo: 32 }));
  trace_exit!();
}
