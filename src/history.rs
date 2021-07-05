use std::io;

use crate::command::{Command, Commands};
use crate::util;

#[derive(Default)]
pub struct History<'a> {
    commands: Commands<'a>,
}

impl<'a> History<'a> {
    fn add_command(&mut self, command: Command<'a>, timestamp: i64) {
        let entry = self.commands.entry(timestamp).or_default();
        // Don't add duplicates.
        if !entry.iter().any(|a| a.first_line == command.first_line) {
            entry.push(command);
        }
    }

    fn try_add_command(&mut self, command: Command<'a>, timestamp: Option<i64>) -> Command<'a> {
        if command.line_count > 0 {
            self.add_command(command, timestamp.unwrap());
            Command::default()
        } else {
            command
        }
    }

    pub fn add(&mut self, data: &'a str) {
        // No timestamp to start with, and an empty command.
        let mut current_timestamp = None;
        let mut current_command = Command::default();

        for line in data.lines() {
            if let Some(timestamp) = util::parse_timestamp(&line) {
                // This line is a valid timestamp so we should be done
                // parsing a previous command (if any). Try to add it.
                current_command = self.try_add_command(current_command, current_timestamp);
                // This timestamp is now the one we're parsing lines
                // for.
                current_timestamp = Some(timestamp);
            } else if current_timestamp.is_some() && !line.is_empty() {
                // If this line is not a timestamp (but we have a
                // previous one), and this line is not empty, then
                // append it to the current command.
                current_command.add_line(line);
            }
        }

        // Now that we're at the end of the file, try to add the current
        // command.
        self.try_add_command(current_command, current_timestamp);
    }

    pub fn write(&self, mut f: impl io::Write) -> io::Result<()> {
        let mut i = 1;

        for timestamp in self.commands.keys() {
            for command in &self.commands[timestamp] {
                writeln!(
                    f,
                    "{}\t{}\t{}",
                    i,
                    util::format_timestamp(*timestamp),
                    command
                )?;
                i += 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;

    #[test]
    fn history() {
        let mut history = History::default();
        history.add("#1624823400\ncommand2\n#1624823395\ncommand1");
        let mut stdout = Vec::new();
        history.write(&mut stdout).unwrap();

        assert_eq!(
            from_utf8(&stdout).unwrap(),
            "1\t2021-06-27 20:49:55\tcommand1\n2\t2021-06-27 20:50:00\tcommand2\n"
        );
    }
}
