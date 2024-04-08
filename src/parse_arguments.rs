use crate::{history_related, create};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
    verbatim_doc_comment
)]
struct Arguments {
    path: Vec<String>,
    /// Clear history.
    #[clap(short, long)]
    clear_history: bool,
    /// Delete history interactively.
    #[clap(short = 'D', long)]
    delete_history: bool,
    /// Create directories instead of files.
    #[clap(short, long)]
    directory: bool,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
    /// Show history.
    #[clap(short, long)]
    show_history: bool,
}

pub fn parse_arguments() {
    let arguments = Arguments::parse();

    if arguments.show_history {
       history_related::show_history();
    }

    if arguments.delete_history {
        history_related::delete_history(arguments.quiet);
    }

    if arguments.clear_history {
        history_related::clear_history(arguments.quiet);
    }

    if arguments.directory {
        create::create_dirs(&arguments.path, arguments.quiet);
    } else {
        create::create_files(&arguments.path, arguments.quiet);
    }
}
