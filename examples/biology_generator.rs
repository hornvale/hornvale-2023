#![allow(unused_imports)]

use anyhow::Error;
use hornvale::biology::Genus;
use hornvale::biology::GenusCollection;
use hornvale::biology::Species;
use hornvale::biology::SpeciesCollection;
use hornvale::*;

fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  let mut genus_collection = GenusCollection::default();
  let mut species_collection = SpeciesCollection::default();
  let goblin_genus = Genus {
    id: "goblin".to_string(),
    name: "Goblin".to_string(),
  };
  let goblin_species = Species {
    id: "goblin".to_string(),
    name: "goblin".to_string(),
    genus: "goblin".to_string(),
  };
  genus_collection
    .genera
    .insert(goblin_genus.id.clone(), goblin_genus.clone());
  species_collection
    .species
    .insert(goblin_species.id.clone(), goblin_species.clone());
  genus_collection.save()?;
  species_collection.save()?;
  Ok(())
}
