mod create;
mod history;
mod history_related;
mod parse_arguments;
mod prelude;

use crate::{parse_arguments::parse_arguments, prelude::xdg_data_home};
use std::{fs, path, process};

fn main() {
    let history_dir = xdg_data_home().join("wmk").join("history");

    if let Err(e) = fs::create_dir_all::<&path::Path>(history_dir.as_ref()) {
        eprintln!("error: {}", e);
        process::exit(1);
    }

    parse_arguments(history_dir);
}
