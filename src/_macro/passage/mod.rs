#[macro_export]
macro_rules! create_passage {
  (@inner $data: expr, $from: expr, $to: expr, $direction: expr) => {{
    use $crate::map::Passage;
    use $crate::map::PassageDestination;
    use $crate::entity::RoomId;
    if let Some(has_passages) = $data.has_passages.get_mut($from) {
      has_passages.set_passage(
        $direction,
        Some(Passage {
          direction: $direction.to_owned(),
          from: RoomId($from.id()),
          to: PassageDestination::Room(RoomId($to.id())),
        }),
      );
    }
  }};
  ($data: expr, $from: expr, $to: expr, $direction: expr, $bidirectional: expr) => {{
    create_passage!(@inner $data, $from, $to, $direction);
    if $bidirectional {
      create_passage!(@inner $data, $to, $from, &$direction.get_inverse());
    }
  }};
}

#[macro_export]
macro_rules! get_passages {
  ($data: expr, $room: expr) => {{
    let mut result = None;
    if let Some(passages) = $data.has_passages.get($room) {
      result = Some(passages.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_passage_to {
  ($data: expr, $room: expr, $direction: expr) => {{
    let mut result = None;
    if let Some(passages) = get_passages!($data, $room) {
      if let Some(passage) = passages.get_passage_to($direction) {
        result = Some(passage.to_owned());
      }
    }
    result
  }};
}
