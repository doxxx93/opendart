//! Error types.

use std::fmt;

/// A boxed error from a decoding step (JSON, XML, ZIP or a field value).
pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Errors returned by [`Client`](crate::Client) methods.
///
/// Every variant names the endpoint that failed, such as `list.json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP request failed or returned a non-success HTTP status.
    ///
    /// The request URL is removed from the error because it carries the API
    /// key as a query parameter.
    #[error("{endpoint}: HTTP request failed")]
    #[non_exhaustive]
    Http {
        /// The endpoint that was requested, e.g. `list.json`.
        endpoint: &'static str,
        /// The reqwest error, without the request URL.
        #[source]
        source: reqwest::Error,
    },

    /// OpenDART answered with a `status` other than `000`.
    #[error("{endpoint}: OpenDART returned status {status}: {message}")]
    #[non_exhaustive]
    Api {
        /// The endpoint that was requested, e.g. `list.json`.
        endpoint: &'static str,
        /// The status code, e.g. [`Status::RateLimited`] for `020`.
        status: Status,
        /// The message OpenDART sent with the status, in Korean.
        message: String,
    },

    /// The response body did not have the expected shape.
    #[error("{endpoint}: failed to decode the response")]
    #[non_exhaustive]
    Decode {
        /// The endpoint whose response failed to decode, e.g. `list.json`.
        endpoint: &'static str,
        /// The underlying parse error.
        #[source]
        source: BoxError,
    },
}

impl Error {
    /// The endpoint that failed, e.g. `list.json`.
    pub fn endpoint(&self) -> &'static str {
        match self {
            Error::Http { endpoint, .. }
            | Error::Api { endpoint, .. }
            | Error::Decode { endpoint, .. } => endpoint,
        }
    }

    /// The OpenDART status, if this is an [`Error::Api`].
    pub fn status(&self) -> Option<&Status> {
        match self {
            Error::Api { status, .. } => Some(status),
            _ => None,
        }
    }

    /// Whether OpenDART rejected the request because the key ran out of
    /// quota (status `020`).
    ///
    /// The limit is per key, so a caller holding several keys can switch to
    /// another one. See the crate docs for the limits.
    pub fn is_rate_limited(&self) -> bool {
        matches!(self.status(), Some(Status::RateLimited))
    }

    pub(crate) fn decode(endpoint: &'static str, source: impl Into<BoxError>) -> Self {
        Error::Decode {
            endpoint,
            source: source.into(),
        }
    }

    /// Wraps a reqwest error, dropping the URL because it holds the key.
    pub(crate) fn http(endpoint: &'static str, source: reqwest::Error) -> Self {
        Error::Http {
            endpoint,
            source: source.without_url(),
        }
    }
}

/// An OpenDART `status` code other than `000` (success).
///
/// New codes may be added in minor releases; codes this crate does not know
/// yet arrive as [`Status::Unknown`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Status {
    /// `010`: the API key is not registered.
    UnregisteredKey,
    /// `011`: the API key is registered but suspended.
    DisabledKey,
    /// `012`: the request came from an IP address the key does not allow.
    IpNotAllowed,
    /// `013`: no data matched the request.
    NoData,
    /// `014`: the requested file does not exist.
    FileNotFound,
    /// `020`: the key exceeded its request limit.
    RateLimited,
    /// `021`: too many companies in one request (at most 100).
    TooManyCompanies,
    /// `100`: a request parameter has an invalid value.
    InvalidParameter,
    /// `101`: the request is not allowed.
    InvalidAccess,
    /// `800`: the service is down for maintenance.
    Maintenance,
    /// `900`: an undefined error on the OpenDART side.
    Undefined,
    /// `901`: the key expired because the account's personal-data
    /// retention period ended.
    KeyExpired,
    /// A code this crate does not know yet.
    Unknown(String),
}

impl Status {
    /// Maps a raw `status` value. Returns `None` for `000` (success).
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "000" => return None,
            "010" => Status::UnregisteredKey,
            "011" => Status::DisabledKey,
            "012" => Status::IpNotAllowed,
            "013" => Status::NoData,
            "014" => Status::FileNotFound,
            "020" => Status::RateLimited,
            "021" => Status::TooManyCompanies,
            "100" => Status::InvalidParameter,
            "101" => Status::InvalidAccess,
            "800" => Status::Maintenance,
            "900" => Status::Undefined,
            "901" => Status::KeyExpired,
            other => Status::Unknown(other.to_owned()),
        })
    }

    /// The raw three-digit code, e.g. `"020"`.
    pub fn code(&self) -> &str {
        match self {
            Status::UnregisteredKey => "010",
            Status::DisabledKey => "011",
            Status::IpNotAllowed => "012",
            Status::NoData => "013",
            Status::FileNotFound => "014",
            Status::RateLimited => "020",
            Status::TooManyCompanies => "021",
            Status::InvalidParameter => "100",
            Status::InvalidAccess => "101",
            Status::Maintenance => "800",
            Status::Undefined => "900",
            Status::KeyExpired => "901",
            Status::Unknown(code) => code,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_round_trip() {
        for code in [
            "010", "011", "012", "013", "014", "020", "021", "100", "101", "800", "900", "901",
            "999",
        ] {
            let status = Status::from_code(code).unwrap();
            assert_eq!(status.code(), code);
        }
        assert_eq!(Status::from_code("000"), None);
        assert_eq!(
            Status::from_code("999"),
            Some(Status::Unknown("999".into()))
        );
    }

    #[test]
    fn rate_limit_is_detectable() {
        let err = Error::Api {
            endpoint: "list.json",
            status: Status::RateLimited,
            message: "요청 제한을 초과하였습니다.".into(),
        };
        assert!(err.is_rate_limited());
        assert_eq!(
            err.to_string(),
            "list.json: OpenDART returned status 020: 요청 제한을 초과하였습니다."
        );
    }
}
