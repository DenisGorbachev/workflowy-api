use crate::{Client, GetNodesRequestRef};
use std::clone::Clone;
use std::error::Error;
use std::sync::LazyLock;

macro_rules! static_env_var {
    ($vis:vis $name:ident) => {
        $vis static $name: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::env::var(stringify!($name)).expect(concat!(stringify!($name), " must be set")));
    };
}

static_env_var!(TEST_WORKFLOWY_API_KEY);

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new((*TEST_WORKFLOWY_API_KEY).clone()));

#[tokio::test]
async fn must_get_nodes() -> Result<(), Box<dyn Error>> {
    let request = GetNodesRequestRef {
        parent_id: "None",
    };

    let nodes = CLIENT.get_nodes(request).await?;
    assert!(!nodes.nodes.is_empty());

    Ok(())
}
