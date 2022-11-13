use super::Genus;
use anyhow::Error;
use std::collections::HashMap;
use std::fs;

pub mod constants;
pub use constants::*;

/// The `GenusCollection` type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GenusCollection {
  pub genera: HashMap<String, Genus>,
}

impl GenusCollection {
  /// Load genera.
  pub fn load() -> Result<Self, Error> {
    let data = fs::read_to_string(&PATH_TO_DATA)?;
    let result: Self = serde_yaml::from_str(&data)?;
    Ok(result)
  }

  /// Save genera.
  pub fn save(&self) -> Result<(), Error> {
    let data = serde_yaml::to_string(&self)?;
    fs::write(PATH_TO_DATA, data)?;
    Ok(())
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;
  use anyhow::Error;

  #[test]
  pub fn test_genus_collection_serialization() -> Result<(), Error> {
    init();
    let goblin = Genus {
      id: "goblin".to_string(),
      name: "Goblin".to_string(),
    };
    let mut data = GenusCollection::default();
    data.genera.insert(goblin.id.clone(), goblin.clone());
    data.save()?;
    let data2 = GenusCollection::load()?;
    let goblin2 = data2.genera.get("goblin").unwrap();
    assert_eq!(goblin, *goblin2);
    Ok(())
  }
}
