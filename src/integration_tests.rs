use crate::{Client, ClientGetNodesError, ClientNewError, GetNodesRequest, ParentId};
use errgonomic::handle;
use static_env_var::static_env_var;
use std::clone::Clone;
use thiserror::Error;

static_env_var!(TEST_WORKFLOWY_API_KEY);

#[tokio::test]
async fn must_get_nodes() -> Result<(), MustGetNodesError> {
    use MustGetNodesError::*;
    let client = handle!(Client::new((*TEST_WORKFLOWY_API_KEY).clone()), ClientNewFailed);
    let request = GetNodesRequest::<'static>::from(ParentId::Root);
    let nodes = handle!(client.get_nodes(&request).await, GetNodesFailed, request);
    assert!(!nodes.nodes.is_empty());
    Ok(())
}

#[derive(Error, Debug)]
pub enum MustGetNodesError {
    #[error("failed to create Workflowy API client")]
    ClientNewFailed { source: ClientNewError },
    #[error("failed to get Workflowy nodes")]
    GetNodesFailed { source: ClientGetNodesError, request: GetNodesRequest<'static> },
}
