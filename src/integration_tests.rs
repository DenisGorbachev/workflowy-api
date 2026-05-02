use crate::{Client, GetNodesRequest, ParentId};
use static_env_var::static_env_var;
use std::clone::Clone;
use std::error::Error;

static_env_var!(TEST_WORKFLOWY_API_KEY);

#[tokio::test]
async fn must_get_nodes() -> Result<(), Box<dyn Error>> {
    let client = Client::new((*TEST_WORKFLOWY_API_KEY).clone())?;
    let request = GetNodesRequest::from(ParentId::Root);
    let nodes = client.get_nodes(&request).await?;
    assert!(!nodes.nodes.is_empty());

    Ok(())
}
