use super::Species;
use anyhow::Error;
use std::collections::HashMap;
use std::fs;

pub mod constants;
pub use constants::*;

/// The `SpeciesCollection` type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SpeciesCollection {
  pub species: HashMap<String, Species>,
}

impl SpeciesCollection {
  /// Load species.
  pub fn load() -> Result<Self, Error> {
    let data = fs::read_to_string(&PATH_TO_DATA)?;
    let result: Self = serde_yaml::from_str(&data)?;
    Ok(result)
  }

  /// Save species.
  pub fn save(&self) -> Result<(), Error> {
    let data = serde_yaml::to_string(&self)?;
    fs::write(PATH_TO_DATA, data)?;
    Ok(())
  }
}

#[cfg(test)]
pub mod test {

  use super::super::Species;
  use super::*;
  use crate::test::*;
  use anyhow::Error;

  #[test]
  pub fn test_species_collection_serialization() -> Result<(), Error> {
    init();
    let goblin = Species {
      id: "goblin".to_string(),
      name: "goblin".to_string(),
      genus: "goblin".to_string(),
    };
    let mut data = SpeciesCollection::default();
    data.species.insert(goblin.id.clone(), goblin.clone());
    data.save()?;
    let data2 = SpeciesCollection::load()?;
    let goblin2 = data2.species.get("goblin").unwrap();
    assert_eq!(goblin, *goblin2);
    Ok(())
  }
}
