use pretty_env_logger::env_logger::builder;

use hornvale::io::repl::Repl;

fn main() {
  let _ = builder().try_init();
  let mut repl = Repl::default();
  repl.run().unwrap();
}
