use crate::effect::Effect as EffectObject;

/// The `EffectEvent` type.
///
/// This represents an effect on the game world.
#[derive(Clone, Debug)]
pub struct Effect {
  pub effect: EffectObject,
}
