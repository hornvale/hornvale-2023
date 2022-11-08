use async_std::stream;
use futures::prelude::*;
use rustyline_async::SharedWriter;
use specs::prelude::*;
use specs::shrev::EventChannel;
use std::collections::VecDeque;
use std::time::Duration;

use crate::ecs::components::register_components;
use crate::ecs::event_channels::insert_event_channels;
use crate::ecs::event_channels::InputEvent;
use crate::ecs::resources::*;
use crate::ecs::systems::get_new_dispatcher;
use crate::ecs::systems::run_initial_systems;

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
  /// The system dispatcher.
  #[derivative(Debug = "ignore")]
  pub dispatcher: Dispatcher<'static, 'static>,
  /// Messages.
  #[derivative(Debug = "ignore")]
  pub messages: VecDeque<String>,
  /// Raw output channel.
  #[derivative(Debug = "ignore")]
  pub output: SharedWriter,
}

impl Game {
  /// Initialize ECS.
  pub fn new() -> Self {
    let mut ecs = World::new();
    insert_resources(&mut ecs);
    insert_event_channels(&mut ecs);
    register_components(&mut ecs);
    run_initial_systems(&mut ecs);
    let output = {
      let output_resource = ecs.read_resource::<OutputResource>();
      output_resource.0.as_ref().unwrap().clone()
    };
    let dispatcher = get_new_dispatcher(&mut ecs);
    let messages = VecDeque::new();
    Self {
      ecs,
      dispatcher,
      messages,
      output,
    }
  }

  /// Run.
  pub async fn run(&mut self) -> Result<(), Error> {
    let mut input_resource = self.ecs.write_resource::<InputResource>();
    // Probably move to a prompt system?  Or not?  IDK.
    let stdin = input_resource.0.as_mut().unwrap();
    // If we need to print to stdout.
    let _stdout = self.output.clone();

    let mut tick_timer = stream::interval(Duration::from_millis(TICK_INTERVAL));
    let mut periodic_timer2 = stream::interval(Duration::from_millis(900));

    loop {
      futures::select! {
        _ = tick_timer.next().fuse() => {
          self.dispatcher.dispatch(&self.ecs);
        }
        _ = periodic_timer2.next().fuse() => {
          /*
          let new_messages = self
            .ecs
            .write_resource::<MessagesResource>()
            .0
            .drain(..)
            .collect::<Vec<String>>();
          for message in new_messages {
            self.messages.push_front(message);
          }
          while self.messages.len() > 500 {
            self.messages.pop_back();
          }
          */
          // writeln!(stdout, "{}", "Second timer went off!".to_string().trim()).await?;
        }
        command = stdin.readline().fuse() => match command {
          Ok(line) => {
            let line = line.trim();
            stdin.add_history_entry(line.to_owned());
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
    Self::new()
  }
}
