use std::collections::BTreeMap;
use std::fmt;

pub type Commands<'a> = BTreeMap<i64, Vec<Command<'a>>>;

#[derive(Default)]
pub struct Command<'a> {
    pub first_line: Option<&'a str>,
    pub line_count: u16,
}

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first_line = self.first_line.as_ref().unwrap();
        if self.line_count > 1 {
            write!(
                formatter,
                "{}\x1b[0m [... TRUNCATED: {} more line(s) ...]",
                first_line,
                self.line_count - 1
            )
        } else {
            write!(formatter, "{}\x1b[0m", first_line)
        }
    }
}

impl<'a> Command<'a> {
    pub fn add_line(&mut self, line: &'a str) {
        if self.line_count == 0 {
            self.first_line = Some(line);
        }

        self.line_count += 1;
    }
}
