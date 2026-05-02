use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(transparent)]
pub struct Id(pub String);
