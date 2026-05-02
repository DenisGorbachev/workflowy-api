use crate::{GetNodesRequestRef, GetNodesResponse};
use crate::{Key, Limiter};
use derive_more::{From, Into};
use errgonomic::handle;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use thiserror::Error;
use url::Url;
use url_macro::url;

// The trailing slash is required for Url::join to work properly
pub static BASE_URL: LazyLock<Url> = LazyLock::new(|| url!("https://workflowy.com/api/v1/"));

#[derive(From, Into, Eq, PartialEq, Clone, Debug)]
pub struct Client {
    pub base: Url,
    pub key: Key,
    pub limiter: Limiter,
}

impl Client {
    const NODES_PATH: &str = "nodes";

    pub fn new(key: impl Into<Key>) -> Self {
        Self::from(key.into())
    }

    pub async fn get_nodes(&self, request: &GetNodesRequestRef<'_>) -> Result<GetNodesResponse, ClientGetNodesError> {
        use ClientGetNodesError::*;
        let url = self
            .base
            .join(Self::NODES_PATH)
            .expect("always succeeds because `path` is valid");
        let http = handle!(reqwest::Client::builder().build(), BuildHttpClientFailed);
        let result = http
            .get(url)
            .bearer_auth(&self.key)
            .query(&request)
            .send()
            .await;
        let response = handle!(result, SendFailed);
        let nodes = handle!(Self::handle(response).await, HandleFailed);
        Ok(nodes)
    }

    pub async fn handle<T>(response: reqwest::Response) -> Result<T, HandleError>
    where
        T: DeserializeOwned,
    {
        use HandleError::*;
        let status = response.status();
        if status.is_success() {
            let response = response.json::<T>().await;
            Ok(handle!(response, DecodeResponseFailed))
        } else {
            let body = response.text().await;
            let body = handle!(body, ReadBodyFailed, status);
            Err(CheckStatusFailed {
                status,
                body,
            })
        }
    }
}

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("Workflowy returned status '{status}': '{body}'")]
    CheckStatusFailed { status: reqwest::StatusCode, body: String },
    #[error("failed to read Workflowy error response body for status '{status}'")]
    ReadBodyFailed { source: reqwest::Error, status: reqwest::StatusCode },
    #[error("failed to decode Workflowy response")]
    DecodeResponseFailed { source: reqwest::Error },
}

impl From<Key> for Client {
    fn from(key: Key) -> Self {
        Self {
            base: BASE_URL.clone(),
            key,
            limiter: Limiter,
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientGetNodesError {
    #[error("failed to build HTTP client")]
    BuildHttpClientFailed { source: reqwest::Error },
    #[error("failed to send get nodes request")]
    SendFailed { source: reqwest::Error },
    #[error("failed to handle get nodes response")]
    HandleFailed { source: HandleError },
}
