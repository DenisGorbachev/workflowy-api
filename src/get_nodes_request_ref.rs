#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct GetNodesRequestRef<'a> {
    #[serde(rename = "parent_id")]
    pub parent_id: &'a str,
}
