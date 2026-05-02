use crate::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GetNodesResponse {
    pub nodes: Vec<Node>,
}
