/// The `TokenLiteral` enum.
#[derive(Clone, Debug, Display, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Literal {
  String(String),
  Number(i32),
  Nil,
}
