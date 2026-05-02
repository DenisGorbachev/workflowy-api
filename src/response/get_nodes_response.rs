use crate::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNodesResponse {
    pub nodes: Vec<Node>,
}
