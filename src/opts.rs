//! Set up options.

use clap::{App, Arg};
use failure::Error;
use std::env;
use std::path::{Path, PathBuf};

fn app() -> App<'static, 'static> {
    App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("root")
                .long("root")
                .help("Run using the given path as a configuration root.")
                .takes_value(true),
        ).arg(
            Arg::with_name("force")
                .long("force")
                .help("When updating configuration, force the update."),
        ).arg(
            Arg::with_name("non-interactive")
                .long("non-interactive")
                .help("Force to run in non-interactive mode."),
        )
}

/// Parse command-line options.
pub fn opts() -> Result<Opts, Error> {
    let matches = app().get_matches();

    let mut opts = Opts::default();

    opts.root = matches.value_of("root").map(PathBuf::from);
    opts.force = matches.is_present("force");
    opts.non_interactive = matches.is_present("force");

    Ok(opts)
}

/// A set of parsed options.
#[derive(Default)]
pub struct Opts {
    /// The root at which the project is running from.
    pub root: Option<PathBuf>,
    /// Force update.
    pub force: bool,
    /// Run in non-interactive mode.
    pub non_interactive: bool,
}

impl Opts {
    /// Find root directory based on options.
    pub fn root(&self) -> Result<PathBuf, Error> {
        match self.root.as_ref() {
            Some(root) => Ok(root.to_owned()),
            None => {
                if let Some(path) = env::args().next() {
                    Ok(Path::new(&path).canonicalize()?)
                } else {
                    Ok(env::current_dir()?)
                }
            }
        }
    }
}