use pretty_env_logger::env_logger::builder;

use hornvale::commands::error::Error as CommandError;
use hornvale::io::error::Error as IoError;
use hornvale::io::repl::Repl;
use hornvale::parsing::error::Error as ParsingError;

fn main() {
  let _ = builder().try_init();
  let mut repl = Repl::default();
  match repl.run() {
    Ok(()) => {},
    Err(error) => match error {
      IoError::ParsingError(ParsingError::CommandError(inner_error)) => match inner_error {
        CommandError::UserExitError => println!("{}", inner_error),
        _ => println!("Error: {}", inner_error),
      },
      _ => println!("Error: {}", error),
    },
  }
}
