use specs::prelude::*;
use specs::shrev::EventChannel;

use super::event_channels::*;

pub mod create_map;
pub use create_map::CreateMap as CreateMapSystem;
pub mod create_player;
pub use create_player::CreatePlayer as CreatePlayerSystem;
pub mod process_action;
pub use process_action::ProcessAction as ProcessActionSystem;
pub mod process_command;
pub use process_command::ProcessCommand as ProcessCommandSystem;
pub mod process_input;
pub use process_input::ProcessInput as ProcessInputSystem;
pub mod process_output;
pub use process_output::ProcessOutput as ProcessOutputSystem;
pub mod tick;
pub use tick::Tick as TickSystem;

pub fn run_initial_systems(ecs: &mut World) {
  (CreatePlayerSystem {}).run_now(ecs);
  (CreateMapSystem {}).run_now(ecs);
}

pub fn get_new_dispatcher(ecs: &mut World) -> Dispatcher<'static, 'static> {
  let process_input_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<InputEvent>>().register_reader();
    ProcessInputSystem { reader_id }
  };
  let process_output_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<OutputEvent>>().register_reader();
    ProcessOutputSystem { reader_id }
  };
  let process_command_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<CommandEvent>>().register_reader();
    ProcessCommandSystem { reader_id }
  };
  let process_action_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ActionEvent>>().register_reader();
    ProcessActionSystem { reader_id }
  };
  let tick_system = TickSystem {};
  let dispatcher = DispatcherBuilder::new()
    .with(process_input_system, "process_input", &[])
    .with(process_command_system, "process_command", &["process_input"])
    .with(
      process_action_system,
      "process_action",
      &["process_input", "process_command"],
    )
    .with(
      process_output_system,
      "process_output",
      &["process_input", "process_command", "process_action"],
    )
    .with(
      tick_system,
      "tick",
      &["process_input", "process_command", "process_action", "process_output"],
    )
    .build();
  dispatcher
}

/*
use specs::shrev::EventChannel;

use crate::event::{ActionEvent, CommandEvent, ErrorEvent, InputEvent, OutputEvent, ScriptEvent};

use crate::system::*;

pub fn run_initial_systems(ecs: &mut World) {
  (CreatePlayerSystem {}).run_now(ecs);
  (CreateWorldSystem {}).run_now(ecs);
}

pub fn get_new_dispatcher<'a, 'b>(ecs: &mut World) -> Dispatcher<'a, 'b> {
  let process_input_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<InputEvent>>().register_reader();
    ProcessInputSystem { reader_id }
  };
  let process_command_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<CommandEvent>>().register_reader();
    ProcessCommandSystem { reader_id }
  };
  let process_action_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ActionEvent>>().register_reader();
    ProcessActionSystem { reader_id }
  };
  let process_error_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ErrorEvent>>().register_reader();
    ProcessErrorSystem { reader_id }
  };
  let process_output_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<OutputEvent>>().register_reader();
    ProcessOutputSystem { reader_id }
  };
  let process_script_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ScriptEvent>>().register_reader();
    ProcessScriptSystem { reader_id }
  };
  let dispatcher = DispatcherBuilder::new()
    .with(ExperimentSystem {}, "experiment", &[])
    .with(process_error_system, "process_error", &[])
    .with(process_output_system, "process_output", &[])
    .with(process_input_system, "process_input", &[])
    .with(process_command_system, "process_command", &["process_input"])
    .with(process_action_system, "process_action", &["process_command"])
    .with(process_script_system, "process_script", &["process_action"])
    .with(TickSystem {}, "tick", &[])
    .build();
  dispatcher
}
*/
