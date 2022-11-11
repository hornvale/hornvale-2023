use specs::prelude::*;
use specs::shrev::EventChannel;

pub mod action;
pub use action::Action as ActionEvent;
pub mod command;
pub use command::Command as CommandEvent;
pub mod effect;
pub use effect::Effect as EffectEvent;
pub mod input;
pub use input::Input as InputEvent;
pub mod output;
pub use output::Output as OutputEvent;
pub mod script;
pub use script::Script as ScriptEvent;

pub fn insert_event_channels(ecs: &mut World) {
  ecs.insert(EventChannel::<ActionEvent>::new());
  ecs.insert(EventChannel::<CommandEvent>::new());
  ecs.insert(EventChannel::<EffectEvent>::new());
  ecs.insert(EventChannel::<InputEvent>::new());
  ecs.insert(EventChannel::<OutputEvent>::new());
  ecs.insert(EventChannel::<ScriptEvent>::new());
}
