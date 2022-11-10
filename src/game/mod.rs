use async_std::stream;
use futures::prelude::*;
use rustyline_async::SharedWriter;
use specs::prelude::*;
use specs::shrev::EventChannel;
use std::time::Duration;

use crate::ecs::components::register_components;
use crate::ecs::event_channels::insert_event_channels;
use crate::ecs::event_channels::InputEvent;
use crate::ecs::resources::*;
use crate::ecs::systems::*;

pub mod constants;
use constants::*;
pub mod error;
use error::Error;
pub use error::Error as GameError;

/// The `Game` struct.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Game {
  /// The world state, generally all of the information available.
  #[derivative(Debug = "ignore")]
  pub ecs: World,
  /// The tick dispatcher.
  #[derivative(Debug = "ignore")]
  pub tick_dispatcher: Dispatcher<'static, 'static>,
  /// The second dispatcher.
  #[derivative(Debug = "ignore")]
  pub second_dispatcher: Dispatcher<'static, 'static>,
  /// Raw output channel.
  #[derivative(Debug = "ignore")]
  pub output: SharedWriter,
}

impl Game {
  /// Initialize ECS.
  pub fn new(seed: &str) -> Self {
    let mut ecs = World::new();
    insert_resources(&mut ecs, seed);
    insert_event_channels(&mut ecs);
    register_components(&mut ecs);
    let tick_dispatcher = get_tick_dispatcher(&mut ecs);
    let second_dispatcher = get_second_dispatcher(&mut ecs);
    let output = {
      let output_resource = ecs.read_resource::<OutputResource>();
      output_resource.0.as_ref().unwrap().clone()
    };
    Self {
      ecs,
      tick_dispatcher,
      second_dispatcher,
      output,
    }
  }

  /// Run.
  pub async fn run(&mut self) -> Result<(), Error> {
    run_initial_systems(&mut self.ecs);
    // This is how we read input.
    let mut input_resource = self.ecs.write_resource::<InputResource>();
    // Probably move to a prompt system?  Or not?  IDK.
    let stdin = input_resource.0.as_mut().unwrap();
    // If we need to print without sending it through the whole thing.
    let _stdout = self.output.clone();
    // It'd be interesting to store this in a resource and possibly modify it
    // on the fly.  Very FRP.  Much signal.
    let mut tick_timer = stream::interval(Duration::from_millis(TICK_INTERVAL));
    // The second timer.
    let mut second_timer = stream::interval(Duration::from_secs(1));
    // Main game loop, such as it is.
    loop {
      // Select the next future to complete.
      futures::select! {
        _ = tick_timer.next().fuse() => {
          // Each tick, run all of the systems.  We could have multiple
          // dispatchers, each running a subset of the systems, and scheduled
          // differently.
          self.tick_dispatcher.dispatch(&self.ecs);
        }
        _ = second_timer.next().fuse() => {
          // Each second, run the secondary systems.
          self.second_dispatcher.dispatch(&self.ecs);
        }
        command = stdin.readline().fuse() => match command {
          Ok(line) => {
            // We could conceivably be parsing some commands (like Quit, etc)
            // from here rather than sending them through the system, but I
            // think that's a bad architectural decision.
            let line = line.trim();
            stdin.add_history_entry(line.to_owned());
            // We could write "input" in other places.  This might be a way
            // (however unsophisticated) of building macros into the UI.
            self.ecs
              .write_resource::<EventChannel<InputEvent>>()
              .single_write(InputEvent {
                input: line.to_owned(),
              });
          },
          Err(error) => return Err(error.into()),
        },
      }
    }
    // Ok(())
  }
}

impl Default for Game {
  fn default() -> Self {
    Self::new("goat boy")
  }
}
