use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutMode {
    Bullets,
    Todo,
    H1,
    H2,
    H3,
    CodeBlock,
    QuoteBlock,
}
