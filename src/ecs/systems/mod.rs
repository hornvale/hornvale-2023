use specs::prelude::*;
use specs::shrev::EventChannel;

use super::event_channels::*;

pub mod create_map;
pub use create_map::CreateMap as CreateMapSystem;
pub mod create_player;
pub use create_player::CreatePlayer as CreatePlayerSystem;
pub mod experiment;
pub use experiment::Experiment as ExperimentSystem;
pub mod action_processor;
pub use action_processor::ActionProcessor as ActionProcessorSystem;
pub mod command_processor;
pub use command_processor::CommandProcessor as CommandProcessorSystem;
pub mod effect_processor;
pub use effect_processor::EffectProcessor as EffectProcessorSystem;
pub mod input_processor;
pub use input_processor::InputProcessor as InputProcessorSystem;
pub mod output_processor;
pub use output_processor::OutputProcessor as OutputProcessorSystem;
pub mod tick;
pub use tick::Tick as TickSystem;

pub fn run_initial_systems(ecs: &mut World) {
  (CreatePlayerSystem {}).run_now(ecs);
  (CreateMapSystem {}).run_now(ecs);
}

pub fn get_tick_dispatcher(ecs: &mut World) -> Dispatcher<'static, 'static> {
  let input_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<InputEvent>>().register_reader();
    InputProcessorSystem { reader_id }
  };
  let output_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<OutputEvent>>().register_reader();
    OutputProcessorSystem { reader_id }
  };
  let command_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<CommandEvent>>().register_reader();
    CommandProcessorSystem { reader_id }
  };
  let action_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ActionEvent>>().register_reader();
    ActionProcessorSystem { reader_id }
  };
  let effect_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<EffectEvent>>().register_reader();
    EffectProcessorSystem { reader_id }
  };
  let tick_system = TickSystem {};
  let experiment_system = ExperimentSystem {};
  let dispatcher = DispatcherBuilder::new()
    .with(experiment_system, "experiment", &[])
    .with(input_processor_system, "input_processor", &[])
    .with(command_processor_system, "command_processor", &["input_processor"])
    .with(action_processor_system, "action_processor", &["command_processor"])
    .with(effect_processor_system, "effect_processor", &["action_processor"])
    .with(output_processor_system, "output_processor", &["effect_processor"])
    .with(tick_system, "tick", &["output_processor"])
    .build();
  dispatcher
}

pub fn get_second_dispatcher(_ecs: &mut World) -> Dispatcher<'static, 'static> {
  let dispatcher = DispatcherBuilder::new().build();
  dispatcher
}
