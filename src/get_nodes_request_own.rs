use crate::{GetNodesRequestRef, ParentId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetNodesRequestOwn {
    pub parent_id: ParentId<'static>,
}

impl From<GetNodesRequestRef<'_>> for GetNodesRequestOwn {
    fn from(request: GetNodesRequestRef<'_>) -> Self {
        let parent_id = request.parent_id.into_owned();
        Self {
            parent_id,
        }
    }
}

impl<'a> From<&'a GetNodesRequestOwn> for GetNodesRequestRef<'a> {
    fn from(request: &'a GetNodesRequestOwn) -> Self {
        GetNodesRequestRef {
            parent_id: request.parent_id.as_ref(),
        }
    }
}
