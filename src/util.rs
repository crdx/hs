use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
use std::fmt;

pub fn parse_timestamp(line: &str) -> Option<i64> {
    if line.len() < 11 {
        return None;
    }

    let line = line.strip_prefix('#')?;

    if !line.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    line.parse().ok()
}

struct FormattedTimestamp(DateTime<Local>);

impl fmt::Display for FormattedTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Not using formatting string '%F %T' here for small efficiency
        // gains.
        write!(
            f,
            "{:0>2}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
            self.0.year(),
            self.0.month(),
            self.0.day(),
            self.0.hour(),
            self.0.minute(),
            self.0.second()
        )
    }
}

pub fn format_timestamp(timestamp: i64) -> impl fmt::Display {
    FormattedTimestamp(Local.timestamp(timestamp, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_format() {
        assert_eq!(
            format_timestamp(1624823395).to_string(),
            "2021-06-27 20:49:55"
        );
    }

    #[test]
    fn good_timestamp_parse() {
        assert_eq!(parse_timestamp("#1624823395"), Some(1624823395));
    }

    #[test]
    fn bad_timestamp_parse() {
        assert_eq!(parse_timestamp("potato"), None);
    }

    #[test]
    fn empty_timestamp_parse() {
        assert_eq!(parse_timestamp(""), None);
    }

    #[test]
    fn multibyte_timestamp_parse() {
        assert_eq!(parse_timestamp("#🔥"), None);
    }

    #[test]
    fn overflow_timestamp_parse() {
        assert_eq!(parse_timestamp("#01189998819991197250003"), None);
    }
}
