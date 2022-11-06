pub mod direction;
pub use direction::error::Error as DirectionError;
pub use direction::Direction;
pub mod passage;
pub use passage::destination::Destination as PassageDestination;
pub use passage::Passage;
