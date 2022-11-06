use specs::prelude::*;
use std::collections::VecDeque;

use crate::ecs::components::register_components;
use crate::ecs::event_channels::insert_event_channels;
use crate::ecs::resources::*;
use crate::ecs::systems::get_new_dispatcher;
use crate::ecs::systems::run_initial_systems;

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
}

impl Game {
  /// Initialize ECS.
  pub fn new() -> Self {
    let mut ecs = World::new();
    insert_resources(&mut ecs);
    insert_event_channels(&mut ecs);
    register_components(&mut ecs);
    run_initial_systems(&mut ecs);
    let dispatcher = get_new_dispatcher(&mut ecs);
    let messages = VecDeque::new();
    Self {
      ecs,
      dispatcher,
      messages,
    }
  }

  /// Tick.
  pub fn tick(&mut self) -> Result<(), Error> {
    self.dispatcher.dispatch(&self.ecs);
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
    Ok(())
  }
}

impl Default for Game {
  fn default() -> Self {
    Self::new()
  }
}
