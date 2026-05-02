use crate::ParentId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetNodesRequestRef<'a> {
    pub parent_id: ParentId<'a>,
}
