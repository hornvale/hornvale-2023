use crate::astronomy::moon::Moon;

pub mod constants;
pub mod constraints;
pub mod error;

/// The `Moons` object is a wrapper around a list of `Moon` objects.
#[derive(Clone, Debug, PartialEq)]
pub struct Moons {
  pub moons: Vec<Moon>,
}
