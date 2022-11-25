use crate::ecs::entity::{EntityId, PlayerId};
use anyhow::Error as AnyError;

/// The `ParserData` trait.
pub trait ParserData {
  /// Retrieve the player ID.
  fn get_player_id(&self) -> Result<PlayerId, AnyError>;
  /// Retrieve a list of nouns.
  fn get_nouns(&self) -> Result<Vec<(String, EntityId)>, AnyError>;
  /// Retrieve a list of genitives.
  fn get_genitives(&self) -> Result<Vec<String>, AnyError>;
  /// Retrieve a list of adjectives.
  fn get_adjectives(&self) -> Result<Vec<String>, AnyError>;
}
