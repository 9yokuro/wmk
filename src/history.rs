use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    date: DateTime<Local>,
    is_dir: bool,
    path: path::PathBuf,
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = format!("{}", self.date.format("%d %b %R")).cyan();

        let state = if self.path.exists() {
            "  exists  ".green()
        } else {
            "not exists".red()
        };

        let path = if self.is_dir {
            self.path.to_string_lossy().blue().to_string()
        } else {
            self.path.to_string_lossy().to_string()
        };

        write!(f, "{} {} {}", date, state, path)
    }
}

impl History {
    pub const fn new(date: DateTime<Local>, is_dir: bool, path: path::PathBuf) -> Self {
        Self { date, is_dir, path }
    }

    pub fn read(file: fs::File) -> serde_json::Result<Self> {
        serde_json::from_reader(file)
    }

    pub fn write(&self, file: fs::File) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(file, &self)
    }

    pub fn history_file(&self) -> String {
        let date = self.date.format("%FT%T%.9f");
        let file_name = &self.path.file_name().unwrap().to_string_lossy().to_string();

        format!("{}-{}.json", date, file_name)
    }
}
