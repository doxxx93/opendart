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
    /// `opendart-rs/<version>` User-Agent, a 10 second connect timeout, a
    /// 30 second read timeout and no `Referer` header. The read timeout
    /// fails a request when no data arrives for 30 seconds; there is no
    /// limit on the whole request, so large downloads over slow links still
    /// finish.
    ///
    /// To change these, or to set a proxy, TLS options or a shared
    /// connection pool, use [`Client::from_reqwest`].
    ///
    /// # Panics
    ///
    /// Panics if the TLS backend cannot be initialized, like
    /// [`reqwest::Client::new`].
    pub fn new(api_key: impl Into<String>) -> Self {
        let http = builder()
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
    ///
    /// Turn off the `Referer` header with
    /// [`reqwest::ClientBuilder::referer`]`(false)`: the request URL holds
    /// the API key, and reqwest sends it as the `Referer` when it follows a
    /// redirect, also to another host.
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

/// The settings of [`Client::new`]. `referer(false)` because reqwest would
/// otherwise send the request URL, key included, to wherever a redirect
/// leads.
fn builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(CONNECT_TIMEOUT)
        .read_timeout(READ_TIMEOUT)
        .referer(false)
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

    /// A server that answers one request with `response` and hands back
    /// what it received.
    fn serve_once(response: String) -> (u16, std::thread::JoinHandle<String>) {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buf = [0; 1024];
            while !request.ends_with(b"\r\n\r\n") {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                request.extend_from_slice(&buf[..n]);
            }
            stream.write_all(response.as_bytes()).unwrap();
            String::from_utf8_lossy(&request).into_owned()
        });
        (port, handle)
    }

    /// The key is in the query string, so a `Referer` would carry it to
    /// the host a redirect points at.
    #[tokio::test]
    async fn redirects_do_not_carry_the_key_to_another_host() {
        let ok = "HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}";
        let (other, other_seen) = serve_once(ok.to_owned());
        let redirect = format!(
            "HTTP/1.1 302 Found\r\nlocation: http://localhost:{other}/elsewhere\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"
        );
        let (first, first_seen) = serve_once(redirect);
        let http = builder().no_proxy().build().unwrap();
        let response = http
            .get(format!(
                "http://127.0.0.1:{first}/api/list.json?crtfc_key=secret-key"
            ))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
        assert!(first_seen.join().unwrap().contains("crtfc_key=secret-key"));
        let other_request = other_seen.join().unwrap();
        assert!(
            other_request.starts_with("GET /elsewhere"),
            "{other_request}"
        );
        assert!(!other_request.contains("secret-key"), "{other_request}");
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
