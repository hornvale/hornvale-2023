/// The `Tick` resource.
///
/// This is just a counter that increments each tick.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[repr(transparent)]
pub struct Tick(pub u64);
