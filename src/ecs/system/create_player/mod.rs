use specs::prelude::*;

use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::ecs::resource::*;

pub struct CreatePlayer {}

#[derive(SystemData)]
pub struct CreatePlayerData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub camera_resource: Write<'a, CameraResource>,
  pub has_brief_description: WriteStorage<'a, HasBriefDescription>,
  pub has_gender: WriteStorage<'a, HasGender>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
  pub has_name: WriteStorage<'a, HasName>,
  pub is_a_player: WriteStorage<'a, IsAPlayer>,
  pub is_an_actor: WriteStorage<'a, IsAnActor>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for CreatePlayer {
  type SystemData = CreatePlayerData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    if data.player_resource.0.is_none() {
      let player = create_player!(data, Gender::Male);
      data.player_resource.0 = Some(PlayerId(player.id()));
      data.camera_resource.0 = Some(EntityId(player.id()));
    }
  }
}
