use specs::prelude::*;

use super::super::components::*;
use super::super::entity::*;
use super::super::resources::*;

pub struct CreatePlayer {}

#[derive(SystemData)]
pub struct CreatePlayerData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub camera_resource: Write<'a, CameraResource>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_name: WriteStorage<'a, HasName>,
  pub is_a_being: WriteStorage<'a, IsABeing>,
  pub is_a_player: WriteStorage<'a, IsAPlayer>,
}

// This system should normally only be run at startup.
impl<'a> System<'a> for CreatePlayer {
  type SystemData = CreatePlayerData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    if data.player_resource.0.is_none() {
      let player = create_player!(data);
      data.player_resource.0 = Some(PlayerId(player.id()));
      data.camera_resource.0 = Some(EntityId(player.id()));
    }
  }
}
