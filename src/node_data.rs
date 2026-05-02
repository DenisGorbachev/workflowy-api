use crate::LayoutMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeData {
    pub layout_mode: LayoutMode,
}
