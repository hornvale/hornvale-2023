use async_std::stream;
use futures::prelude::*;
use rustyline_async::SharedWriter;
use specs::prelude::*;
use specs::shrev::EventChannel;
use std::io::Write;
use std::time::Duration;

use crate::ecs::component::register_components;
use crate::ecs::event::insert_event_channels;
use crate::ecs::event::InputEvent;
use crate::ecs::resource::*;
use crate::ecs::system::*;

pub mod _constant;
use _constant::*;
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
  /// Every tenth tick (roughly).
  #[derivative(Debug = "ignore")]
  pub deca_tick_dispatcher: Dispatcher<'static, 'static>,
  /// Every hundredth tick (roughly).
  #[derivative(Debug = "ignore")]
  pub hecto_tick_dispatcher: Dispatcher<'static, 'static>,
  /// Every thousandth tick (roughly).
  #[derivative(Debug = "ignore")]
  pub kilo_tick_dispatcher: Dispatcher<'static, 'static>,
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
    let deca_tick_dispatcher = get_deca_tick_dispatcher(&mut ecs);
    let hecto_tick_dispatcher = get_hecto_tick_dispatcher(&mut ecs);
    let kilo_tick_dispatcher = get_kilo_tick_dispatcher(&mut ecs);
    let output = {
      let output_resource = ecs.read_resource::<OutputResource>();
      output_resource.0.as_ref().unwrap().clone()
    };
    Self {
      ecs,
      tick_dispatcher,
      deca_tick_dispatcher,
      hecto_tick_dispatcher,
      kilo_tick_dispatcher,
      output,
    }
  }

  /// Run.
  pub async fn run(&mut self) -> Result<(), Error> {
    run_initial_systems(&mut self.ecs);
    // If we need to print without sending it through the whole thing.
    let mut stdout = self.output.clone();
    // It'd be interesting to store this in a resource and possibly modify it
    // on the fly.  Very FRP.  Much signal.
    let mut tick_timer = stream::interval(Duration::from_millis(TICK_INTERVAL));
    // A local tick counter; just for performing less frequent operations.
    let mut tick: u64 = 0;
    // Main game loop, such as it is.
    loop {
      // Maintain after every tick.  This enables the use of the lazy systems,
      // which should make it easier to have simple, concise systems.
      self.ecs.maintain();
      // This is how we read input.
      let mut input_resource = self.ecs.write_resource::<InputResource>();
      // Probably move to a prompt system?  Or not?  IDK.
      let stdin = input_resource.0.as_mut().unwrap();
      // Select the next future to complete.
      futures::select! {
        _ = tick_timer.next().fuse() => {
          // Each tick, run all of the systems.  We could have multiple
          // dispatchers, each running a subset of the systems, and scheduled
          // differently.
          self.tick_dispatcher.dispatch(&self.ecs);
          tick += 1;
          if tick % 10 == 0 {
            self.deca_tick_dispatcher.dispatch(&self.ecs);
            if tick % 100 == 0 {
              self.hecto_tick_dispatcher.dispatch(&self.ecs);
              if tick % 1000 == 0 {
                self.kilo_tick_dispatcher.dispatch(&self.ecs);
              }
            }
          }
        }
        command = stdin.readline().fuse() => match command {
          Ok(line) => {
            // We could conceivably be parsing some commands (like Quit, etc)
            // from here rather than sending them through the system, but I
            // think that's a bad architectural decision.
            let line = line.trim();
            writeln!(stdout, "> {}\n", line)?;
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
