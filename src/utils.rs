use colored::Colorize;
use filey::{Filey, Result};
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
