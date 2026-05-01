#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetNodesRequestRef<'a> {
    #[cfg_attr(feature = "serde", serde(rename = "parent_id"))]
    pub parent_id: &'a str,
}
