pub mod common;
use common::entity::Entity;
use common::sense_event::SenseEvent;

#[test]
fn test_entity() {
  println!("entity: {:?}", Entity {});
  println!("red light on: {:?}", SenseEvent::RedLightStatus(true));
  println!("red light off: {:?}", SenseEvent::RedLightStatus(false));
}
