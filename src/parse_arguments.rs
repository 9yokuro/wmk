use crate::{create, history_related};
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
    /// Do not create a history file.
    #[clap(short = 'R', long)]
    no_record: bool,
    /// Do not print log messages.
    #[clap(short, long)]
    quiet: bool,
    /// Show history.
    #[clap(short, long)]
    show_history: bool,
}

pub struct Options {
    no_record: bool,
    quiet: bool,
}

impl Options {
    pub const fn new(no_record: bool, quiet: bool) -> Self {
        Self { no_record, quiet }
    }

    pub const fn no_record(&self) -> bool {
        self.no_record
    }

    pub const fn quiet(&self) -> bool {
        self.quiet
    }
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

    let options = Options::new(arguments.no_record, arguments.quiet);

    if arguments.directory {
        create::create_dirs(&arguments.path, &options);
    } else {
        create::create_files(&arguments.path, &options);
    }
}
