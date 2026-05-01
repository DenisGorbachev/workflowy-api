use crate::Node;

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GetNodesResponse {
    pub nodes: Vec<Node>,
}
