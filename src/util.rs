use chrono::{Datelike, Local, TimeZone, Timelike};
use std::fmt;

pub fn parse_timestamp(line: &str) -> Option<i64> {
    if line.len() < 11 {
        return None;
    }

    let line = line.strip_prefix('#')?;

    if line.bytes().all(|b| b.is_ascii_digit()) {
        line.parse().ok()
    } else {
        None
    }
}

pub fn format_timestamp(timestamp: i64) -> impl fmt::Display {
    let a = Local.timestamp(timestamp, 0);

    // Not using formatting string '%F %T' here for small efficiency
    // gains.
    format!(
        "{:0>2}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
        a.year(),
        a.month(),
        a.day(),
        a.hour(),
        a.minute(),
        a.second()
    )
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
