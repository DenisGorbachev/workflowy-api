use crate::{GetNodesRequestRef, ParentId};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct GetNodesRequestOwn {
    #[cfg_attr(feature = "serde", serde(rename = "parent_id"))]
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
