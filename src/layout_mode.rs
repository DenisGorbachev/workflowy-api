#[derive(Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub enum LayoutMode {
    Bullets,
    Todo,
    H1,
    H2,
    H3,
    CodeBlock,
    QuoteBlock,
    Unknown(String),
}

impl LayoutMode {
    pub fn as_str(&self) -> &str {
        use LayoutMode::*;

        match self {
            Bullets => "bullets",
            Todo => "todo",
            H1 => "h1",
            H2 => "h2",
            H3 => "h3",
            CodeBlock => "code-block",
            QuoteBlock => "quote-block",
            Unknown(value) => value.as_str(),
        }
    }
}

impl From<String> for LayoutMode {
    fn from(value: String) -> Self {
        use LayoutMode::*;

        match value.as_str() {
            "bullets" => Bullets,
            "todo" => Todo,
            "h1" => H1,
            "h2" => H2,
            "h3" => H3,
            "code-block" => CodeBlock,
            "quote-block" => QuoteBlock,
            _ => Unknown(value),
        }
    }
}

impl From<&str> for LayoutMode {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LayoutMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LayoutMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::from)
    }
}
