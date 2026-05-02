use crate::ParentId;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetNodesRequest<'p> {
    pub parent_id: Cow<'p, ParentId<'p>>,
}

impl<'p> GetNodesRequest<'p> {}

impl<'p> From<ParentId<'p>> for GetNodesRequest<'p> {
    fn from(parent_id: ParentId<'p>) -> Self {
        Self {
            parent_id: Cow::Owned(parent_id),
        }
    }
}

impl<'p> From<&'p ParentId<'p>> for GetNodesRequest<'p> {
    fn from(parent_id: &'p ParentId<'p>) -> Self {
        Self {
            parent_id: Cow::Borrowed(parent_id),
        }
    }
}
