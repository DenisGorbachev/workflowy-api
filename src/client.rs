use crate::{GetNodesRequest, GetNodesResponse};
use crate::{Key, Limiter};
use derive_more::{From, Into};
use errgonomic::handle;
use reqwest::Client as HttpClient;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use secrecy::ExposeSecret as _;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use thiserror::Error;
use url::Url;
use url_macro::url;

// The trailing slash is required for Url::join to work properly
pub static BASE_URL: LazyLock<Url> = LazyLock::new(|| url!("https://workflowy.com/api/v1/"));

#[derive(From, Into, Clone, Debug)]
pub struct Client {
    pub inner: HttpClient,
    pub base: Url,
    pub limiter: Limiter,
}

impl Client {
    const NODES_PATH: &str = "nodes";

    pub fn new(key: impl Into<Key>) -> Result<Self, ClientNewError> {
        use ClientNewError::*;
        let key = key.into();
        Ok(handle!(Self::try_from(key), TryFromKeyFailed))
    }

    pub async fn get_nodes(&self, request: &GetNodesRequest<'_>) -> Result<GetNodesResponse, ClientGetNodesError> {
        use ClientGetNodesError::*;
        let url = self
            .base
            .join(Self::NODES_PATH)
            .expect("always succeeds because `NODES_PATH` is a valid relative URL path");
        let result = self.inner.get(url).query(&request).send().await;
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

#[derive(Error, Debug)]
pub enum ClientNewError {
    #[error("failed to create Workflowy API client from key")]
    TryFromKeyFailed { source: ConvertKeyToClientError },
}

impl From<HttpClient> for Client {
    fn from(inner: HttpClient) -> Self {
        Self {
            inner,
            base: BASE_URL.clone(),
            limiter: Limiter,
        }
    }
}

impl TryFrom<Key> for Client {
    type Error = ConvertKeyToClientError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        use ConvertKeyToClientError::*;
        let header_value_raw = format!("Bearer {}", key.expose_secret());
        let mut header_value = handle!(HeaderValue::try_from(header_value_raw), HeaderValueTryFromFailed, key);
        header_value.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_value);
        let inner = HttpClient::builder().default_headers(headers).build();
        let inner = handle!(inner, BuildHttpClientFailed);
        Ok(Self::from(inner))
    }
}

#[derive(Error, Debug)]
pub enum ConvertKeyToClientError {
    #[error("failed to convert Workflowy API key into an authorization header value")]
    HeaderValueTryFromFailed { source: reqwest::header::InvalidHeaderValue, key: Key },
    #[error("failed to build HTTP client")]
    BuildHttpClientFailed { source: reqwest::Error },
}

#[derive(Error, Debug)]
pub enum ClientGetNodesError {
    #[error("failed to send get nodes request")]
    SendFailed { source: reqwest::Error },
    #[error("failed to handle get nodes response")]
    HandleFailed { source: HandleError },
}
