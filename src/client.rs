#[cfg(feature = "serde")]
use crate::{GetNodesRequestRef, GetNodesResponse};
use crate::{Key, Limiter};
use derive_more::{From, Into};
use thiserror::Error;

#[cfg(feature = "serde")]
const NODES_URL: &str = "https://workflowy.com/api/v1/nodes";

#[derive(From, Into, Eq, PartialEq, Clone, Debug)]
pub struct Client {
    pub key: Key,
    pub limiter: Limiter,
}

impl Client {
    pub fn new(key: impl Into<Key>) -> Self {
        Self::from(key.into())
    }

    #[cfg(feature = "serde")]
    pub async fn get_nodes(&self, request: GetNodesRequestRef<'_>) -> Result<GetNodesResponse, GetNodesError> {
        use GetNodesError::*;

        let http = reqwest::Client::builder()
            .build()
            .map_err(|source| BuildHttpClient {
                source,
            })?;
        let response = http
            .get(NODES_URL)
            .bearer_auth(&self.key)
            .query(&request)
            .send()
            .await
            .map_err(|source| Send {
                source,
            })?;
        let status = response.status();

        if status.is_success() {
            response
                .json::<GetNodesResponse>()
                .await
                .map_err(|source| Decode {
                    source,
                })
        } else {
            let body = response.text().await.map_err(|source| ReadErrorBody {
                source,
            })?;
            Err(UnexpectedStatus {
                status,
                body,
            })
        }
    }
}

impl From<Key> for Client {
    fn from(key: Key) -> Self {
        Self {
            key,
            limiter: Limiter,
        }
    }
}

#[derive(Error, Debug)]
pub enum GetNodesError {
    #[error("failed to build HTTP client")]
    BuildHttpClient { source: reqwest::Error },
    #[error("failed to send get nodes request")]
    Send { source: reqwest::Error },
    #[error("Workflowy returned status {status}: {body}")]
    UnexpectedStatus { status: reqwest::StatusCode, body: String },
    #[error("failed to read Workflowy error response body")]
    ReadErrorBody { source: reqwest::Error },
    #[error("failed to decode get nodes response")]
    Decode { source: reqwest::Error },
}
