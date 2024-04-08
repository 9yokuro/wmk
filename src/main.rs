mod create;
mod history;
mod history_related;
mod parse_arguments;
mod prelude;

use crate::{parse_arguments::parse_arguments, prelude::xdg_data_home};
use std::{fs, io};

fn main() {
    initialize();
    parse_arguments();
}

fn initialize() {
    let history_dir = xdg_data_home().join("wmk");

    if let Err(e) = fs::create_dir_all(history_dir) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            eprintln!("error: {}", e);
        }
    }
}
