use crate::{NodeData, NodeId, WorkflowyTimestamp};

#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub note: Option<String>,
    pub priority: u64,
    pub data: NodeData,
    #[cfg_attr(feature = "serde", serde(rename = "createdAt"))]
    pub created_at: WorkflowyTimestamp,
    #[cfg_attr(feature = "serde", serde(rename = "modifiedAt"))]
    pub modified_at: WorkflowyTimestamp,
    #[cfg_attr(feature = "serde", serde(rename = "completedAt"))]
    pub completed_at: Option<WorkflowyTimestamp>,
}
