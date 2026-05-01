use crate::{NodeData, NodeId, WorkflowyTimestamp};

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub priority: u64,
    pub data: NodeData,
    #[serde(rename = "createdAt")]
    pub created_at: WorkflowyTimestamp,
    #[serde(rename = "modifiedAt")]
    pub modified_at: WorkflowyTimestamp,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<WorkflowyTimestamp>,
}
