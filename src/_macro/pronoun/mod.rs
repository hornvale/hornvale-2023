
#[macro_export]
macro_rules! subject_pronoun {
  ($data: expr, $entity: expr) => {{
    use $crate::gender::Gender;
    use Gender::*;
    match get_gender!($data, $entity) {
      Male => "he",
      Female => "she",
      Genderless => "it",
      Nonbinary => "they",
      Unknown => "they",
    }
  }};
}

#[macro_export]
macro_rules! object_pronoun {
  ($data: expr, $entity: expr) => {{
    use $crate::gender::Gender;
    use Gender::*;
    match get_gender!($data, $entity) {
      Male => "him",
      Female => "her",
      Genderless => "it",
      Nonbinary => "them",
      Unknown => "them",
    }
  }};
}

#[macro_export]
macro_rules! possessive_pronoun {
  ($data: expr, $entity: expr) => {{
    use $crate::gender::Gender;
    use Gender::*;
    match get_gender!($data, $entity) {
      Male => "his",
      Female => "hers",
      Genderless => "its",
      Nonbinary => "theirs",
      Unknown => "theirs",
    }
  }};
}

#[macro_export]
macro_rules! possessive_adjective {
  ($data: expr, $entity: expr) => {{
    use $crate::gender::Gender;
    use Gender::*;
    match get_gender!($data, $entity) {
      Male => "his",
      Female => "hers",
      Genderless => "its",
      Nonbinary => "their",
      Unknown => "their",
    }
  }};
}

#[macro_export]
macro_rules! reflexive_pronoun {
  ($data: expr, $entity: expr) => {{
    use $crate::gender::Gender;
    use Gender::*;
    match get_gender!($data, $entity) {
      Male => "himself",
      Female => "herself",
      Genderless => "itself",
      Nonbinary => "themself",
      Unknown => "themself",
    }
  }};
}
