use crate::LayoutMode;

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NodeData {
    #[serde(rename = "layoutMode")]
    pub layout_mode: LayoutMode,
}
