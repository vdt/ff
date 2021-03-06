use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use lazy_static::lazy_static;
use std::env;

const ABOUT: &str = "
Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).";

pub fn app() -> ArgMatches<'static> {
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }

    App::new("ff")
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT)
        .max_term_width(80)
        .arg(
            Arg::with_name("PATTERN")
                .help("Find files whose name (path) matches this substring or the regular expression.")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("ROOT_PATH")
                .help("Path to the directory to search files inside.")
                .index(2)
                .default_value(&WORKING_DIR_PATH)
                .required(false),
        )
        .arg(
            Arg::with_name("search-hidden")
                .help("Search hidden files and directories. By default, hidden files and directories are skipped.")
                .short("H")
                .long("search-hidden"),
        )
        .arg(
            Arg::with_name("case-sensitive")
                .help("Search case sensitively. By default, files are searched case insensitively.")
                .short("s")
                .long("case-sensitive"),
        )
        .get_matches()
}

pub fn working_dir_path() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("."),
    }
}
