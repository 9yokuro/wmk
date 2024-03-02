pub mod actions;
mod history;
mod initialize;
mod parse_arguments;
pub mod utils;

pub use crate::{history::History, parse_arguments::Options};

use crate::{initialize::initialize, parse_arguments::parse_arguments, utils::*};
use std::{path::PathBuf, process::exit};

fn main() {
    let wmk_data_home = PathBuf::from(xdg_data_home()).join("wmk");

    let wmk_data_home_absolutized = &match absolutize(wmk_data_home) {
        Ok(absolutized) => absolutized,
        Err(e) => {
            print_error_message(e);

            exit(1);
        }
    };

    if let Err(e) = initialize(wmk_data_home_absolutized) {
        print_error_message(e);

        exit(1);
    }

    if let Err(e) = parse_arguments(wmk_data_home_absolutized) {
        print_error_message(e);

        exit(1);
    }
}
