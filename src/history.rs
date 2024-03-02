use chrono::{DateTime, Local};
use colored::Colorize;
use filey::{Error::FileyError, Filey, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::{fmt, fs::File, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    date: DateTime<Local>,
    is_directory: bool,
    path: String,
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = format!("{}", self.date().format("%d %b %R")).cyan();

        let state = if self.exists() {
            "  exists  ".green()
        } else {
            "not exists".red()
        };

        let path = if self.is_directory() {
            self.path().blue().to_string()
        } else {
            self.path().to_string()
        };

        write!(f, "{} {} {}", date, state, path)
    }
}

impl History {
    pub fn new<P: AsRef<Path>>(path: P, is_directory: bool) -> Self {
        let date = Local::now();

        let path = path.as_ref().to_string_lossy().to_string();

        Self {
            date,
            is_directory,
            path,
        }
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path).map_err(|e| e.into()).map_err(FileyError)?;

        from_reader(file).map_err(|e| e.into()).map_err(FileyError)
    }

    pub fn date(&self) -> &DateTime<Local> {
        &self.date
    }

    pub fn exists(&self) -> bool {
        Filey::new(&self.path).exists()
    }

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)
            .map_err(|e| e.into())
            .map_err(FileyError)?;

        to_writer_pretty(file, &self)
            .map_err(|e| e.into())
            .map_err(FileyError)
    }
}
