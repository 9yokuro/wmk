use colored::Colorize;
use filey::{Error::FileyError, Filey, Result};
use inquire::Confirm;
use std::{env::var, fmt::Display, path::Path};

pub fn print_error_message<D: Display>(message: D) {
    eprintln!("{}: {}", "error".red().bold(), message);
}

pub fn xdg_data_home() -> String {
    var("XDG_DATA_HOME").unwrap_or_else(|_| "~/.local/share".to_string())
}

pub fn absolutize<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(Filey::new(path).expand_user()?.absolutize()?.to_string())
}

pub fn confirm<D: Display>(message: D) -> Result<bool> {
    Confirm::new(message.to_string().as_str())
        .with_default(false)
        .prompt()
        .map_err(|e| e.into())
        .map_err(FileyError)
}
