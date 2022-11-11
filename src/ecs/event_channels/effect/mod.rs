use crate::effect::Effect as EffectObject;

/// The `EffectEvent` type.
///
/// This represents an effect on the game world.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Effect {
  pub effect: EffectObject,
}
