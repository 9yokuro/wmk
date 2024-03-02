use crate::{utils::*, History, Options};
use colored::Colorize;
use filey::{Error::FileyError, Filey, Result};
use inquire::Select;
use std::{
    fmt::Display,
    io::{stdout, BufWriter, Write},
};
use walkdir::WalkDir;

fn generate_history_path(file_name: &String, history: &History, wmk_data_home: &String) -> String {
    let date = history.date().format("%FT%T%.9f");

    format!("{}/{}-{}.json", wmk_data_home, date, file_name)
}

pub fn create_dir(paths: &Vec<String>, options: &Options, wmk_data_home: &String) {
    for path in paths {
        let path = &match absolutize(path) {
            Ok(absolutized) => absolutized,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        let filey = Filey::new(path);

        if let Err(e) = filey.create_dir() {
            print_error_message(e);

            continue;
        }

        let history = History::new(path, true);

        let history_path =
            generate_history_path(&filey.file_name().unwrap(), &history, wmk_data_home);

        if let Err(e) = history.write(history_path) {
            print_error_message(e);

            continue;
        }

        if !options.quiet() {
            print_create_message(path, true);
        }
    }
}

pub fn create_file(paths: &Vec<String>, options: &Options, wmk_data_home: &String) {
    for path in paths {
        let path = &match absolutize(path) {
            Ok(absolutized) => absolutized,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        let filey = Filey::new(path);

        let parent_dir = Filey::new(filey.parent_dir().unwrap());

        if !parent_dir.exists() {
            if let Err(e) = parent_dir.create_dir() {
                print_error_message(e);

                continue;
            }
        }

        if let Err(e) = filey.create_file() {
            print_error_message(e);

            continue;
        }

        let history = History::new(path, false);

        let history_path =
            generate_history_path(&filey.file_name().unwrap(), &history, wmk_data_home);

        if let Err(e) = history.write(history_path) {
            print_error_message(e);

            continue;
        }

        if !options.quiet() {
            print_create_message(path, false);
        }
    }
}

fn print_create_message<D: Display>(path: D, is_directory: bool) {
    let path = if is_directory {
        path.to_string().blue().to_string()
    } else {
        path.to_string()
    };

    let file_type = if is_directory { "directory" } else { "file" };

    eprintln!("{} {} '{}'", "Created".green().bold(), file_type, path);
}

pub fn show_history(wmk_data_home: &String) {
    if WalkDir::new(wmk_data_home).into_iter().nth(1).is_none() {
        eprintln!("There is no history");

        return;
    }

    let out = stdout();

    let mut out = BufWriter::new(out.lock());

    for path in WalkDir::new(wmk_data_home).into_iter().skip(1) {
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        let history = match History::read(path.path()) {
            Ok(history) => history,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        writeln!(out, "{}", &history).unwrap();
    }
}

pub fn clear_history(options: &Options, wmk_data_home: &String) -> Result<()> {
    let filey = Filey::new(wmk_data_home);

    if !confirm("Clear history?")? {
        eprintln!("Canceled");

        return Ok(());
    }

    filey.remove()?;

    if !options.quiet() {
        eprintln!("{} history", "Cleared".green().bold());
    }

    Ok(())
}

pub fn delete_history(options: &Options, wmk_data_home: &String) -> Result<()> {
    let mut h = vec![];

    for path in WalkDir::new(wmk_data_home).into_iter().skip(1) {
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        let history = match History::read(path.path()) {
            Ok(history) => history,
            Err(e) => {
                print_error_message(e);

                continue;
            }
        };

        h.push(history);
    }

    if h.is_empty() {
        eprintln!("There is no history");

        return Ok(());
    }

    let ans = Select::new("", h.clone())
        .prompt()
        .map_err(|e| e.into())
        .map_err(FileyError)?;

    for history in h {
        if history == ans {
            if confirm("Delete this history?")? {
                Filey::new(generate_history_path(
                    &Filey::new(history.path()).file_name().unwrap(),
                    &history,
                    wmk_data_home,
                ))
                .remove()?;
            } else {
                eprintln!("Canceled");
            }
        }
    }

    if !options.quiet() {
        eprintln!("{} history", "Deleted".green().bold());
    }

    Ok(())
}
