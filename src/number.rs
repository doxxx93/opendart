//! Numeric values, which OpenDART sends as text.

use std::fmt;
use std::str::FromStr;

/// A number as OpenDART writes it, such as `"1,234"`, `"-11,526,297"`,
/// `"8.51"` or `"99.99%"`.
///
/// The text is kept as sent, with surrounding whitespace trimmed, because
/// one column can hold amounts in one row and ratios in the next (the
/// `thstrm` column of [`Dividend`](crate::periodic::Dividend)), and
/// companies sometimes write prose into a numeric column. Convert it with
/// [`parse`](Number::parse) into the type that fits: `i64` for amounts in
/// won and share counts, `f64` or a decimal type such as
/// `rust_decimal::Decimal` for ratios.
///
/// Fields of this type are `Option<Number>`: blank values and a lone `-`,
/// which OpenDART uses for "none", become `None`.
///
/// Equality and hashing compare the text, so the same amount sent as
/// `"566,942,110,000,000"` by one endpoint and `"566942110000000"` by
/// another is not equal; compare parsed values instead.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number(String);

impl Number {
    pub(crate) fn new(text: String) -> Self {
        Number(text)
    }

    /// The text as OpenDART sent it, trimmed.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parses the value with `F`'s [`FromStr`], after removing thousands
    /// separators and a trailing `%`.
    ///
    /// ```
    /// # fn run(total_assets: opendart::Number, ratio: opendart::Number) {
    /// // "566,942,110,000,000"
    /// let won: i64 = total_assets.parse().unwrap();
    /// // "99.99%"
    /// let percent: f64 = ratio.parse().unwrap();
    /// # }
    /// ```
    pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        let text = self.0.strip_suffix('%').unwrap_or(&self.0).trim_end();
        if text.contains(',') {
            text.replace(',', "").parse()
        } else {
            text.parse()
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(text: &str) -> Number {
        Number::new(text.to_owned())
    }

    #[test]
    fn parses_separators_signs_fractions_and_percent() {
        assert_eq!(
            number("566,942,110,000,000").parse::<i64>(),
            Ok(566_942_110_000_000)
        );
        assert_eq!(
            number("-11,526,297,000,000").parse::<i64>(),
            Ok(-11_526_297_000_000)
        );
        assert_eq!(
            number("247684612000000").parse::<i64>(),
            Ok(247_684_612_000_000)
        );
        assert_eq!(number("8.51").parse::<f64>(), Ok(8.51));
        assert_eq!(number("99.99%").parse::<f64>(), Ok(99.99));
        assert_eq!(number("7.70 %").parse::<f64>(), Ok(7.7));
    }

    #[test]
    fn prose_is_kept_and_fails_to_parse() {
        let text = "양도제한주식(RSA)을 '26.1월 지급하기로 한 약정내역";
        let value = number(text);
        assert_eq!(value.as_str(), text);
        assert!(value.parse::<i64>().is_err());
        assert!(number("8.51").parse::<i64>().is_err());
    }
}
