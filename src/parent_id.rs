use std::borrow::Cow;

use ParentId::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ParentId<'a> {
    #[serde(rename = "None")]
    Root,
    Inbox,
    Calendar,
    Today,
    Tomorrow,
    NextWeek,
    #[serde(untagged)]
    Custom(Cow<'a, str>),
}

impl<'a> ParentId<'a> {
    pub fn as_ref(&self) -> ParentId<'_> {
        match self {
            Root => ParentId::Root,
            Inbox => ParentId::Inbox,
            Calendar => ParentId::Calendar,
            Today => ParentId::Today,
            Tomorrow => ParentId::Tomorrow,
            NextWeek => ParentId::NextWeek,
            Custom(parent_id) => ParentId::Custom(Cow::Borrowed(parent_id.as_ref())),
        }
    }

    pub fn into_owned(self) -> ParentId<'static> {
        match self {
            Root => ParentId::Root,
            Inbox => ParentId::Inbox,
            Calendar => ParentId::Calendar,
            Today => ParentId::Today,
            Tomorrow => ParentId::Tomorrow,
            NextWeek => ParentId::NextWeek,
            Custom(parent_id) => ParentId::Custom(Cow::Owned(parent_id.into_owned())),
        }
    }
}

impl From<String> for ParentId<'static> {
    fn from(parent_id: String) -> Self {
        Custom(Cow::Owned(parent_id))
    }
}

impl<'a> From<&'a str> for ParentId<'a> {
    fn from(parent_id: &'a str) -> Self {
        Custom(Cow::Borrowed(parent_id))
    }
}
