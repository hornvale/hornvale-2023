use crate::system::effect_processor::Data as EffectProcessorData;
use anyhow::Error;

pub mod entity;
pub use entity::*;

/// The `Effect` enum.
///
/// An `Effect`, like `Action` and `Command`, should be safe and serializable.
///
/// It represents an objective, measurable effect on the game world.  It is
/// infallible, which is to say that a failure in execution is indicative of a
/// programming error.
///
/// Effects have zero duration; they're considered to occur instantaneously.
///
/// For example, let's say that two goblins are fighting.
/// - Goblin A's AI requests a CastSpell action, which will take 10 time.
/// - Goblin B's AI requests a AttackEntity action, which will take 3 time.
/// - Goblin A's action is enqueued first, with 10 ticks remaining.
/// - Goblin B's action is enqueued on the following tick, with 3 ticks.
/// - Goblin A's action has 9 ticks.
/// - Three ticks later, Goblin B's action completes (tick = 0).  It spawns an
///   effect, which should be completed within that same tick.
/// - The effect is a critical hit to Goblin A, which not only damages his
///   health but disrupts his spellcasting.
/// - Through some process I don't understand yet, Goblin A will be made aware
///   of this on his next AI tick, and things will go from there.
///
/// The point is that the effect takes place completely between the end of one
/// run of `ProcessActionSystem` and the beginning of another run, so within a
/// single tick.
///
/// There are, of course, last-minute checks that need to be performed in this
/// system.  What if three goblins are fighting Goblin A, and one kills him,
/// the second knocks him out, and the third disrupts his spellcasting?
///
/// Well, this is where the programming errors and infallibility comes in.  If
/// these events occur out of order, it could cause a crash, or just be faintly
/// ridiculous.
///
/// `Action`s largely succeed or fail behind the scenes; the `Effect`s can be
/// highly visible, so they need to be coded carefully.
///
/// A heuristic for this might be that `Action` is about game rules, whereas
/// `Effect` is about physical laws.  Don't do weird stuff, and be very fine-
/// grained with creating `Effect`s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Effect {
  /// An entity looks around.
  EntityLooksAround(EntityLooksAround),
  /// An entity looks in a specific direction.
  EntityLooksDirection(EntityLooksDirection),
  /// An entity walks into a room.
  EntityWalksIntoRoom(EntityWalksIntoRoom),
  /// An entity walks out of a room.
  EntityWalksOutOfRoom(EntityWalksOutOfRoom),
  /// An entity's initiative is set to a value.
  EntitySetInitiative(EntitySetInitiative),
}

impl Effect {
  /// Process the effect.
  pub fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    use Effect::*;
    match &self {
      EntityLooksAround(effect) => effect.process(data)?,
      EntityLooksDirection(effect) => effect.process(data)?,
      EntityWalksIntoRoom(effect) => effect.process(data)?,
      EntityWalksOutOfRoom(effect) => effect.process(data)?,
      EntitySetInitiative(effect) => effect.process(data)?,
    }
    Ok(())
  }
}
