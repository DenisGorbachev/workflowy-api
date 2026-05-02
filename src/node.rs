use crate::{NodeData, NodeId, WorkflowyTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub priority: u64,
    pub data: NodeData,
    pub created_at: WorkflowyTimestamp,
    pub modified_at: WorkflowyTimestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<WorkflowyTimestamp>,
}
