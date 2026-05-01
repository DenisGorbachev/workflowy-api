use subtype::subtype_string;

subtype_string! {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(transparent)]
    pub struct NodeId(pub String);
}
