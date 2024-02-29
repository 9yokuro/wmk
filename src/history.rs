use chrono::{DateTime, Local};
use colored::Colorize;
use filey::{Error::FileyError, Filey, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::{fs::File, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    date: DateTime<Local>,
    exists: bool,
    is_directory: bool,
    path: String,
}

impl History {
    pub fn new<P: AsRef<Path>>(path: P, is_directory: bool) -> Self {
        let date = Local::now();

        let exists = Filey::new(&path).exists();

        let path = path.as_ref().to_string_lossy().to_string();

        Self {
            date,
            exists,
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
        self.exists
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

pub fn format(history: &History) -> String {
    let date = format!("{}", history.date().format("%d %b %R")).cyan();

    let state = if history.exists() {
        "exists".green()
    } else {
        "not exists".red()
    };

    let path = if history.is_directory() {
        history.path().blue().to_string()
    } else {
        history.path().to_string()
    };

    format!("{} {} {}", date, state, path)
}
