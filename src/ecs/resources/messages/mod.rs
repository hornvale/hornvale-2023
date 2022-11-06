use std::collections::VecDeque;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Messages(pub VecDeque<String>);
