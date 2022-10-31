use hornvale::io::repl::Repl;

fn main() {
  println!("Hello, world!");
  let mut repl = Repl::default();
  repl.run().unwrap();
}
