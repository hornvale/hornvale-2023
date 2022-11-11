use std::collections::HashMap;

use crate::scripting::garbage_collection::reference::Reference;
use crate::scripting::value::Value;

/// The `Table` type.
pub type Table = HashMap<Reference<String>, Value>;
