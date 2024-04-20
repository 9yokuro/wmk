use crate::history::History;
use colored::Colorize;
use inquire::{Confirm, Select};
use std::{
    fs,
    io::{self, Write},
    path,
};
use walkdir::WalkDir;

pub fn clear_history<P>(quiet: bool, history_dir: P)
where
    P: AsRef<path::Path>,
{
    if WalkDir::new(history_dir.as_ref())
        .into_iter()
        .nth(1)
        .is_none()
    {
        eprintln!("There is no history");
        return;
    }

    show_history(history_dir.as_ref());

    let confirmed = match Confirm::new("Clear history?").with_default(false).prompt() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };

    if !confirmed {
        eprintln!("Canceled");
        return;
    }

    if let Err(e) = fs::remove_dir_all(history_dir) {
        eprintln!("error: {}", e);
        return;
    }

    if !quiet {
        eprintln!("{} history", "Cleared".green().bold());
    }
}

pub fn show_history<P>(history_dir: P)
where
    P: AsRef<path::Path>,
{
    if WalkDir::new(history_dir.as_ref())
        .into_iter()
        .nth(1)
        .is_none()
    {
        eprintln!("There is no history");
        return;
    }

    let mut out = io::BufWriter::new(io::stdout().lock());

    for path in WalkDir::new(history_dir).into_iter().skip(1) {
        let path = match path {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        let file = match fs::File::open(path.path()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        let history = match History::read(file) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        writeln!(out, "{}", &history).unwrap();
    }
}

pub fn delete_history<P>(quiet: bool, history_dir: P)
where
    P: AsRef<path::Path>,
{
    if WalkDir::new(history_dir.as_ref())
        .into_iter()
        .nth(1)
        .is_none()
    {
        eprintln!("There is no history");
        return;
    }

    let mut hist = vec![];

    for path in WalkDir::new(history_dir.as_ref()).into_iter().skip(1) {
        let path = match path {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        let file = match fs::File::open(path.path()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        let history = match History::read(file) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        hist.push(history);
    }

    let answer = match Select::new("", hist.clone()).prompt() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };

    hist.retain(|h| h == &answer);

    if !hist.is_empty() {
        let confirmed = match Confirm::new("Delete this history?")
            .with_default(false)
            .prompt()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error: {}", e);
                return;
            }
        };

        if confirmed {
            if let Err(e) = fs::remove_file(history_dir.as_ref().join(answer.history_file())) {
                eprintln!("error: {}", e);
                return;
            }

            if !quiet {
                eprintln!("{} this history", "Deleted".green().bold());
            }
        } else {
            eprintln!("Canceled");
        }
    }
}
