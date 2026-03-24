use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct GCodeParser {
    pub commands: Vec<String>,
}

impl GCodeParser {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if !line.is_empty() && !trimmed.starts_with(';') {
                self.commands.push(trimmed.to_string());
            }
        }
        Ok(())
    }

    pub fn get_next_command(&mut self) -> Option<String> {
        if !self.commands.is_empty() {
            Some(self.commands.remove(0))
        } else {
            None
        }
    }
}