//! The HTTP client.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::Error;

pub(crate) const BASE_URL: &str = "https://opendart.fss.or.kr/api/";
const USER_AGENT: &str = concat!("opendart-rs/", env!("CARGO_PKG_VERSION"));
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// A per-read limit instead of a whole-request one: from a GitHub runner in
/// the US, the 3.6 MB `corpCode.xml` download took longer than 60 seconds.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Query parameters for one request, as `(name, value)` pairs.
pub(crate) type Query = Vec<(&'static str, String)>;

/// Client for the OpenDART API.
///
/// Each endpoint is a method named after the endpoint file in snake case,
/// e.g. [`list`](Client::list) for `list.json` and
/// [`corp_code`](Client::corp_code) for `corpCode.xml`.
///
/// Cloning is cheap: clones share the connection pool and the key. To use
/// several keys, build one `Client` per key from the same
/// [`reqwest::Client`] with [`Client::from_reqwest`].
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    api_key: Arc<str>,
}

impl Client {
    /// Creates a client with a default [`reqwest::Client`]: an
    /// `opendart-rs/<version>` User-Agent, a 10 second connect timeout and
    /// a 30 second read timeout. The read timeout fails a request when no
    /// data arrives for 30 seconds; there is no limit on the whole request,
    /// so large downloads over slow links still finish.
    ///
    /// To change these, or to set a proxy, TLS options or a shared
    /// connection pool, use [`Client::from_reqwest`].
    ///
    /// # Panics
    ///
    /// Panics if the TLS backend cannot be initialized, like
    /// [`reqwest::Client::new`].
    pub fn new(api_key: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(CONNECT_TIMEOUT)
            .read_timeout(READ_TIMEOUT)
            .build()
            .expect("failed to build the reqwest client");
        Self::from_reqwest(http, api_key)
    }

    /// Creates a client on top of an existing [`reqwest::Client`].
    ///
    /// The client must send a `User-Agent` header (see
    /// [`reqwest::ClientBuilder::user_agent`]). OpenDART redirects requests
    /// without one to an error page, which fails as an [`Error::Http`]
    /// redirect error.
    pub fn from_reqwest(http: reqwest::Client, api_key: impl Into<String>) -> Self {
        Client {
            http,
            api_key: api_key.into().into(),
        }
    }

    pub(crate) fn request(&self, endpoint: &'static str, query: &Query) -> reqwest::RequestBuilder {
        self.http
            .get(format!("{BASE_URL}{endpoint}"))
            .query(&[("crtfc_key", &*self.api_key)])
            .query(query)
    }

    /// Sends a GET request and returns the body. OpenDART reports its own
    /// errors inside a `200 OK` body, so callers still decode the status.
    pub(crate) async fn get(
        &self,
        endpoint: &'static str,
        query: &Query,
    ) -> Result<Vec<u8>, Error> {
        let http = |err| Error::http(endpoint, err);
        let response = self.request(endpoint, query).send().await.map_err(http)?;
        let body = response
            .error_for_status()
            .map_err(http)?
            .bytes()
            .await
            .map_err(http)?;
        Ok(body.into())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("api_key", &"<redacted>")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_goes_into_the_query_and_not_into_debug() {
        let client = Client::new("test-key");
        let request = client
            .request("list.json", &vec![("page_no", "2".to_owned())])
            .build()
            .unwrap();
        assert_eq!(
            request.url().as_str(),
            "https://opendart.fss.or.kr/api/list.json?crtfc_key=test-key&page_no=2"
        );
        assert!(!format!("{client:?}").contains("test-key"));
    }

    /// reqwest prints the request URL, query string included, in both
    /// `Display` and `Debug` of its errors.
    #[tokio::test]
    async fn http_errors_do_not_leak_the_key() {
        // Nothing listens on port 1, so the request fails without network.
        let http = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:1").unwrap())
            .build()
            .unwrap();
        let err = Client::from_reqwest(http, "secret-key")
            .company("00126380")
            .await
            .unwrap_err();
        assert!(
            matches!(
                err,
                Error::Http {
                    endpoint: "company.json",
                    ..
                }
            ),
            "{err:?}"
        );

        let mut text = format!("{err} {err:?}");
        let mut source = std::error::Error::source(&err);
        while let Some(inner) = source {
            text.push_str(&format!(" {inner} {inner:?}"));
            source = inner.source();
        }
        assert!(!text.contains("secret-key"), "{text}");
    }
}
