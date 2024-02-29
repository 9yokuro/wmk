pub mod actions;
mod history;
mod initialize;
mod parse_arguments;
pub mod utils;

pub use crate::{history::History, parse_arguments::Options};

use crate::parse_arguments::parse_arguments;

fn main() {
    parse_arguments();
}
