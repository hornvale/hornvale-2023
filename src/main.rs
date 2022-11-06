use pretty_env_logger::env_logger::builder;

use hornvale::game::Game;
use hornvale::game::GameError;

fn main() -> Result<(), GameError> {
  let _ = builder().try_init();
  let mut game = Game::new();
  loop {
    game.tick()?;
  }
}
