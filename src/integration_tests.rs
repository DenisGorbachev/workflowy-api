use crate::{Client, GetNodesRequestRef};
use std::clone::Clone;
use std::error::Error;

macro_rules! static_env_var {
    ($vis:vis $name:ident) => {
        $vis static $name: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::env::var(stringify!($name)).expect(concat!(stringify!($name), " must be set")));
    };
}

static_env_var!(TEST_WORKFLOWY_API_KEY);

#[tokio::test]
async fn must_get_nodes() -> Result<(), Box<dyn Error>> {
    let request = GetNodesRequestRef {
        parent_id: "None",
    };

    let client = Client::new((*TEST_WORKFLOWY_API_KEY).clone())?;
    let nodes = client.get_nodes(&request).await?;
    assert!(!nodes.nodes.is_empty());

    Ok(())
}
