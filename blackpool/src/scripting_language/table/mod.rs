use std::collections::HashMap;

use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::value::Value;

/// The `Table` type.
pub type Table = HashMap<Reference<String>, Value>;
