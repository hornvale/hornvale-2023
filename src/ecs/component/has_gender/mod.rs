use crate::gender::Gender;
use specs::prelude::*;

/// The `HasGender` type.
#[derive(Clone, Component, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct HasGender(pub Gender);
