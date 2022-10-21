pub mod common;
use common::entity::Entity;
use common::perceptual_filter::PerceptualFilter;
use common::sense_event::SenseEvent;
use common::world::World;

#[test]
fn test_entity() {
  let name = "Robot";
  let world = World {
    red_light_status: false,
  };
  let perceptual_filter = PerceptualFilter::new(world);
  let mut entity = Entity::new(name.into(), perceptual_filter);
  println!("entity: {:?}", entity);
  println!("red light on: {:?}", SenseEvent::RedLightStatus(true));
  println!("red light off: {:?}", SenseEvent::RedLightStatus(false));
  println!("perceptual filter: {:?}", entity.perceptual_filter);
  entity.perceptual_filter.world.red_light_status = true;
  assert_eq!(entity.perceptual_filter.get_last().red_light_status, true);
  entity.perceptual_filter.world.red_light_status = false;
  assert_eq!(entity.perceptual_filter.get_last().red_light_status, false);
}
