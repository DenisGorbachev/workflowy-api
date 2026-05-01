use crate::LayoutMode;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct NodeData {
    #[cfg_attr(feature = "serde", serde(rename = "layoutMode"))]
    pub layout_mode: LayoutMode,
}
