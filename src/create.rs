use crate::{history::History, parse_arguments::Options, prelude::*};
use chrono::Local;
use colored::Colorize;
use path_absolutize::Absolutize;
use std::{fs, path};

pub fn create_files(paths: &[String], options: &Options) {
    for path in paths {
        let absolutized = match path::Path::new(path).absolutize() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        if let Err(e) = create_file_all(&absolutized) {
            eprintln!("error: {}", e);
            continue;
        }

        if !options.no_record() {
            let history = History::new(Local::now(), false, absolutized.to_path_buf());
            let history_path = xdg_data_home().join("wmk").join(history.history_file());

            let file = match fs::File::create(history_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error: {}", e);
                    continue;
                }
            };

            if let Err(e) = history.write(file) {
                eprintln!("error: {}", e);
                continue;
            }
        }

        if !options.quiet() {
            eprintln!(
                "{} file '{}'",
                "Created".green().bold(),
                absolutized.to_string_lossy()
            );
        }
    }
}

pub fn create_dirs(paths: &[String], options: &Options) {
    for path in paths {
        let absolutized = match path::Path::new(path).absolutize() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        if let Err(e) = fs::create_dir_all(&absolutized) {
            eprintln!("error: {}", e);
            continue;
        }

        if !options.no_record() {
            let history = History::new(Local::now(), true, absolutized.to_path_buf());
            let history_path = xdg_data_home().join("wmk").join(history.history_file());

            let file = match fs::File::create(history_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error: {}", e);
                    continue;
                }
            };

            if let Err(e) = history.write(file) {
                eprintln!("error: {}", e);
                continue;
            }
        }

        if !options.quiet() {
            eprintln!(
                "{} directory '{}'",
                "Created".green().bold(),
                absolutized.to_string_lossy().blue()
            );
        }
    }
}
