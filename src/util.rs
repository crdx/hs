use chrono::{Datelike, Local, TimeZone, Timelike};
use std::fmt;

pub fn parse_timestamp(line: &str) -> Option<i64> {
    if line.len() < 11 {
        return None;
    }

    let line = line.strip_prefix('#')?;

    if line.bytes().all(|b| b.is_ascii_digit()) {
        Some(line.parse().unwrap())
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
    fn test_format_timestamp() {
        assert_eq!(
            format!("{}", format_timestamp(1624823395)),
            "2021-06-27 20:49:55"
        );
    }

    #[test]
    fn test_good_parse_timestamp() {
        let correct = parse_timestamp("#1624823395");
        assert!(correct.is_some());
        assert_eq!(correct.unwrap(), 1624823395);
    }

    #[test]
    fn test_bad_parse_timestamp() {
        let incorrect = parse_timestamp("potato");
        assert!(incorrect.is_none());
    }
}
