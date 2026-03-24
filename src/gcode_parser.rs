use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip blank lines and semicolon comments commonly used in G-code.
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                self.commands.push_back(trimmed.to_string());
            }
        }

        Ok(())
    }

    /// Returns the next command to execute, or `None` when the queue is empty.
    pub fn get_next_command(&mut self) -> Option<String> {
        self.commands.pop_front()
    }
}
