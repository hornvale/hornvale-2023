use specs::prelude::*;

use super::super::components::*;
use super::super::entity::*;
use super::super::resources::*;

pub struct CreatePlayer {}

#[derive(SystemData)]
pub struct CreatePlayerData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub has_name: WriteStorage<'a, HasName>,
  pub has_description: WriteStorage<'a, HasDescription>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for CreatePlayer {
  type SystemData = CreatePlayerData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    if data.player_resource.0.is_none() {
      let player = data.entities.create();
      data
        .has_name
        .insert(player, HasName("Player".into()))
        .expect("Unable to insert name for player!");
      data
        .has_description
        .insert(
          player,
          HasDescription {
            initial: Some("You're an absolutely unexceptional specimen of whatever species you are.".to_string()),
            brief: "It's you, you idiot!".to_string(),
          },
        )
        .expect("Unable to insert description for player!");
      data.player_resource.0 = Some(PlayerId(player.id()));
    }
  }
}
