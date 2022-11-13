use crate::component::*;
use crate::event::*;
use specs::prelude::*;
use specs::shrev::EventChannel;

pub struct IntentProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
  pub has_intent: ReadStorage<'a, HasIntent>,
  pub lazy_updater: Read<'a, LazyUpdate>,
}

// Check intents and, if possible, dispatch them as actions.
impl<'a> System<'a> for IntentProcessor {
  type SystemData = Data<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    for (entity, mut has_initiative, has_intent) in (&data.entities, &mut data.has_initiative, &data.has_intent).join()
    {
      if has_initiative.0.current > has_intent.0.initiative_cost {
        has_initiative.0.current -= has_intent.0.initiative_cost;
        write_action_event!(data, has_intent.0.action.clone());
        data.lazy_updater.remove::<HasIntent>(entity);
      }
    }
  }
}
