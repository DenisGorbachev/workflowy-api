use crate::{Client, ClientGetNodesError, ClientNewError, GetNodesRequest, ParentId};
use errgonomic::handle;
use static_env_var::static_env_var;
use thiserror::Error;

static_env_var!(WORKFLOWY_API_KEY);

#[tokio::test]
async fn must_get_nodes() -> Result<(), MustGetNodesError> {
    use MustGetNodesError::*;
    let client = handle!(Client::new((*WORKFLOWY_API_KEY).clone()), ClientNewFailed);
    let request = GetNodesRequest::<'static>::from(ParentId::Root);
    let nodes = handle!(client.get_nodes(&request).await, GetNodesFailed, request);
    assert!(!nodes.nodes.is_empty());
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustGetNodesError {
    #[error("failed to create an API client")]
    ClientNewFailed { source: ClientNewError },
    #[error("failed to get nodes")]
    GetNodesFailed { source: ClientGetNodesError, request: GetNodesRequest<'static> },
}
