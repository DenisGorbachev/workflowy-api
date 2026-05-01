use crate::{GetNodesRequestOwn, GetNodesRequestRef, GetNodesResponse};
use crate::{Key, Limiter};
use derive_more::{From, Into};
use errgonomic::handle;
use std::sync::LazyLock;
use thiserror::Error;
use url::Url;
use url_macro::url;

pub static BASE: LazyLock<Url> = LazyLock::new(|| url!("https://workflowy.com/api/v1/"));

const NODES_PATH: &str = "nodes";

#[derive(From, Into, Eq, PartialEq, Clone, Debug)]
pub struct Client {
    pub base: Url,
    pub key: Key,
    pub limiter: Limiter,
}

impl Client {
    pub fn new(key: impl Into<Key>) -> Self {
        Self::from(key.into())
    }

    pub fn base() -> &'static Url {
        &BASE
    }

    pub async fn get_nodes(&self, request: GetNodesRequestRef<'_>) -> Result<GetNodesResponse, ClientGetNodesError> {
        use ClientGetNodesError::*;

        let request = GetNodesRequestOwn::from(request);
        let base = self.base.clone();
        let path = NODES_PATH.to_string();
        let nodes_url = handle!(base.join(path.as_str()), JoinNodesUrlFailed, base, path);
        let http = handle!(reqwest::Client::builder().build(), BuildHttpClientFailed);
        let request_ref = request.as_ref();
        let url_for_http = nodes_url.clone();
        let url_for_error = nodes_url.clone();
        let request_for_error = request.clone();
        let response = http
            .get(url_for_http)
            .bearer_auth(&self.key)
            .query(&request_ref)
            .send()
            .await;
        let response = handle!(response, SendFailed, url: url_for_error, request: request_for_error);
        let status = response.status();

        if status.is_success() {
            let url_for_error = nodes_url.clone();
            let request_for_error = request.clone();
            let response = response.json::<GetNodesResponse>().await;
            Ok(handle!(response, DecodeResponseFailed, url: url_for_error, request: request_for_error))
        } else {
            let url_for_error = nodes_url.clone();
            let request_for_error = request.clone();
            let body = response.text().await;
            let body = handle!(body, ReadErrorBodyFailed, status, url: url_for_error, request: request_for_error);
            Err(UnexpectedStatusInvalid {
                status,
                url: nodes_url,
                request,
                body,
            })
        }
    }
}

impl From<Key> for Client {
    fn from(key: Key) -> Self {
        Self {
            base: Self::base().clone(),
            key,
            limiter: Limiter,
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientGetNodesError {
    #[error("failed to join nodes path '{path}' to Workflowy API base URL '{base}'")]
    JoinNodesUrlFailed { source: url::ParseError, base: Url, path: String },
    #[error("failed to build HTTP client")]
    BuildHttpClientFailed { source: reqwest::Error },
    #[error("failed to send get nodes request to '{url}'")]
    SendFailed { source: reqwest::Error, url: Url, request: GetNodesRequestOwn },
    #[error("Workflowy returned status {status} from '{url}': {body}")]
    UnexpectedStatusInvalid { status: reqwest::StatusCode, url: Url, request: GetNodesRequestOwn, body: String },
    #[error("failed to read Workflowy error response body from '{url}'")]
    ReadErrorBodyFailed { source: reqwest::Error, status: reqwest::StatusCode, url: Url, request: GetNodesRequestOwn },
    #[error("failed to decode get nodes response")]
    DecodeResponseFailed { source: reqwest::Error, url: Url, request: GetNodesRequestOwn },
}
