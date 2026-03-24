use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Upper bound to avoid loading unexpectedly huge G-code files into memory.
const MAX_COMMANDS: usize = 200_000;
/// Upper bound for a single normalized command to avoid abusive line sizes.
const MAX_COMMAND_LENGTH: usize = 4_096;

/// Stores parsed G-code commands and provides sequential access.
///
/// Blank lines and comments (starting with `;`) are removed at load time to keep
/// the execution queue clean for downstream motion-control logic.
pub struct GCodeParser {
    pub commands: VecDeque<String>,
}

impl GCodeParser {
    /// Creates an empty parser queue.
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
        }
    }

    /// Loads and normalizes commands from a `.gcode` file.
    ///
    /// Security/reliability checks:
    /// - Clears previously loaded commands before ingesting the next file.
    /// - Enforces maximum command count and command length to reduce memory abuse risk.
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        self.commands.clear();

        for line in reader.lines() {
            let line = line?;
            if let Some(command) = normalize_line(&line) {
                if command.len() > MAX_COMMAND_LENGTH {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "G-code command exceeds max length of {MAX_COMMAND_LENGTH} characters"
                        ),
                    ));
                }

                if self.commands.len() >= MAX_COMMANDS {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("G-code command count exceeds max of {MAX_COMMANDS}"),
                    ));
                }

                self.commands.push_back(command);
            }
        }

        Ok(())
    }

    /// Returns the next command to execute, or `None` when the queue is empty.
    pub fn get_next_command(&mut self) -> Option<String> {
        self.commands.pop_front()
    }
}

/// Normalizes one raw G-code line:
/// - strips trailing inline comments beginning with `;`
/// - trims whitespace
/// - filters empty result
fn normalize_line(line: &str) -> Option<String> {
    let uncommented = line.split_once(';').map_or(line, |(content, _)| content);
    let trimmed = uncommented.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_line;

    #[test]
    fn normalize_line_removes_comments_and_whitespace() {
        assert_eq!(
            normalize_line("  G0 X10 Y20 ; move home"),
            Some("G0 X10 Y20".to_string())
        );
        assert_eq!(normalize_line("   ; comment only"), None);
        assert_eq!(normalize_line("  \t\n"), None);
    }
}
