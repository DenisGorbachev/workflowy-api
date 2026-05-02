use crate::{GetNodesRequestRef, ParentId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetNodesRequestOwn {
    pub parent_id: ParentId,
}

impl GetNodesRequestOwn {
    pub fn new(parent_id: impl Into<ParentId>) -> Self {
        Self {
            parent_id: parent_id.into(),
        }
    }

    pub fn as_ref(&self) -> GetNodesRequestRef<'_> {
        GetNodesRequestRef {
            parent_id: self.parent_id.as_str(),
        }
    }
}

impl From<GetNodesRequestRef<'_>> for GetNodesRequestOwn {
    fn from(request: GetNodesRequestRef<'_>) -> Self {
        Self::new(request.parent_id)
    }
}

impl<'a> From<&'a GetNodesRequestOwn> for GetNodesRequestRef<'a> {
    fn from(request: &'a GetNodesRequestOwn) -> Self {
        request.as_ref()
    }
}
