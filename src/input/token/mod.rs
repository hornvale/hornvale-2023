use crate::ecs::entity::EntityId;
pub mod literal;
use literal::Literal;
pub use literal::Literal as TokenLiteral;
pub mod r#type;
use r#type::Type;
pub use r#type::Type as TokenType;

/// The `Token` type.
#[derive(Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[display(fmt = "type: {}, lexeme: {}", r#type, lexeme)]
pub struct Token<'input> {
  pub r#type: Type,
  pub lexeme: &'input str,
  pub literal: Option<Literal>,
  pub entity_id: Option<EntityId>,
}

impl<'input> Token<'input> {
  pub fn is_verb(&self) -> bool {
    self.r#type.is_verb()
  }
}
