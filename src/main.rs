use log::LevelFilter;
use simplelog::Config;
use simplelog::WriteLogger;

use hornvale::game::Game;
use hornvale::game::GameError;

#[async_std::main]
async fn main() -> Result<(), GameError> {
  let mut game = Game::new("goat boy2");
  let stdout = game.output.clone();
  WriteLogger::init(LevelFilter::Off, Config::default(), stdout).unwrap();
  game.run().await?;
  Ok(())
}
