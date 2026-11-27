//! The request shared by the major event (`DS005`) and securities
//! registration (`DS006`) endpoints.

use jiff::civil::Date;
use serde::de::DeserializeOwned;

use crate::client::Query;
use crate::{Client, Error, decode};

/// One company and a range of receipt dates (`corp_code`, `bgn_de`,
/// `end_de`), the parameters every [`major_event`](crate::major_event) and
/// [`registration`](crate::registration) endpoint takes. OpenDART has data from 2015 on.
///
/// ```
/// use opendart::DateRangeRequest;
/// use opendart::jiff::civil::date;
///
/// // Samsung Electronics, reports received in 2026.
/// let request = DateRangeRequest::new("00126380", date(2026, 1, 1), date(2026, 12, 31));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateRangeRequest {
    corp_code: String,
    bgn_de: Date,
    end_de: Date,
}

impl DateRangeRequest {
    /// Reports about `corp_code` (8 digits) received from `bgn_de` to
    /// `end_de`, both included.
    pub fn new(corp_code: impl Into<String>, bgn_de: Date, end_de: Date) -> Self {
        DateRangeRequest {
            corp_code: corp_code.into(),
            bgn_de,
            end_de,
        }
    }

    pub(crate) fn query(&self) -> Query {
        vec![
            ("corp_code", self.corp_code.clone()),
            ("bgn_de", decode::format_date(self.bgn_de)),
            ("end_de", decode::format_date(self.end_de)),
        ]
    }
}

impl Client {
    /// [`rows`](Client::rows) for one company and date range.
    pub(crate) async fn date_range_rows<T: DeserializeOwned>(
        &self,
        endpoint: &'static str,
        request: &DateRangeRequest,
    ) -> Result<Vec<T>, Error> {
        self.rows(endpoint, &request.query()).await
    }
}

/// A request for tests that check parameter names.
#[cfg(test)]
pub(crate) fn example() -> DateRangeRequest {
    use jiff::civil::date;
    DateRangeRequest::new("00126380", date(2019, 1, 1), date(2019, 12, 31))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_formats_the_dates() {
        assert_eq!(
            example().query(),
            vec![
                ("corp_code", "00126380".to_owned()),
                ("bgn_de", "20190101".to_owned()),
                ("end_de", "20191231".to_owned()),
            ]
        );
    }
}
