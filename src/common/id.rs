use serde::{Deserialize, Serialize};
use subtype::subtype_string;

subtype_string! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Id(pub String);
}
