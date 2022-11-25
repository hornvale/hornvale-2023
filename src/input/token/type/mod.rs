use anyhow::Error as AnyError;
use std::str::FromStr;

/// The `Type` enum.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Type {
  Adjective,
  Again,
  All,
  Ampersand,
  And,
  Article,
  Asterisk,
  At,
  AtSign,
  BackSlash,
  But,
  Caret,
  Colon,
  Comma,
  Dash,
  Direction,
  Dollar,
  DoubleQuotation,
  Echo,
  Eof,
  Equals,
  Eval,
  ExclamationPoint,
  ForwardSlash,
  Genitive,
  Go,
  GreaterThan,
  Identifier,
  Idle,
  LeftBrace,
  LeftCurlyBrace,
  LeftParenthesis,
  LessThan,
  Literal,
  Look,
  Noun,
  Number,
  On,
  Oops,
  Other,
  Percent,
  Period,
  Pipe,
  Plus,
  Pound,
  Quit,
  RightBrace,
  RightCurlyBrace,
  RightParenthesis,
  Semicolon,
  SingleQuotation,
  String,
  Then,
  Under,
  Underscore,
  Question,
}

impl FromStr for Type {
  type Err = AnyError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    use Type::*;
    match string {
      "again" | "g" => Ok(Again),
      "&" => Ok(Ampersand),
      "all" | "every" | "everything" => Ok(All),
      "and" => Ok(And),
      "a" | "an" | "the" | "some" => Ok(Article),
      "then" => Ok(Then),
      "*" => Ok(Asterisk),
      "at" => Ok(At),
      "@" => Ok(AtSign),
      "\\" => Ok(BackSlash),
      "but" => Ok(But),
      "^" => Ok(Caret),
      ":" => Ok(Colon),
      "," => Ok(Comma),
      "-" => Ok(Dash),
      "northeast" | "north" | "northwest" | "east" | "west" | "southeast" | "south" | "southwest" | "up" | "down"
      | "inside" | "outside" | "in" | "out" | "ne" | "n" | "nw" | "e" | "w" | "se" | "s" | "sw" => Ok(Direction),
      "$" => Ok(Dollar),
      "\"" => Ok(DoubleQuotation),
      "=" => Ok(Equals),
      "eval" => Ok(Eval),
      "!" => Ok(ExclamationPoint),
      "/" => Ok(ForwardSlash),
      "go" | "walk" => Ok(Go),
      ">" => Ok(GreaterThan),
      "idle" | "z" => Ok(Idle),
      "[" => Ok(LeftBrace),
      "{" => Ok(LeftCurlyBrace),
      "(" => Ok(LeftParenthesis),
      "<" => Ok(LessThan),
      "look" | "l" => Ok(Look),
      "atop" | "on" => Ok(On),
      "oops" => Ok(Oops),
      "other" => Ok(Other),
      "%" => Ok(Percent),
      "." => Ok(Period),
      "|" => Ok(Pipe),
      "+" => Ok(Plus),
      "#" => Ok(Pound),
      "quit" => Ok(Quit),
      "]" => Ok(RightBrace),
      "}" => Ok(RightCurlyBrace),
      ")" => Ok(RightParenthesis),
      ";" => Ok(Semicolon),
      "'" => Ok(SingleQuotation),
      "under" | "beneath" => Ok(Under),
      "_" => Ok(Underscore),
      "?" => Ok(Question),
      unknown => Err(anyhow!("Unknown keyword {}", unknown)),
    }
  }
}

impl Type {
  pub fn is_verb(&self) -> bool {
    use Type::*;
    matches!(self, Again | Echo | Eval | Go | Look)
  }
}
