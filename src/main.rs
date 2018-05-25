/*
 * no-modules 1.0.0-alpha (USE WITH CAUTION)
 *
 * Removes node_modules from your filesystem.
 */

extern crate colored;
extern crate getopts;
extern crate walkdir;

use colored::*;
use getopts::Options;
use std::env;
use std::fs;
use walkdir::{DirEntry, WalkDir};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn is_node_modules(entry: &DirEntry) -> bool {
    entry.path().to_str().unwrap().contains("node_modules")
}

fn print_short_usage(program: &str, opts: Options) {
    println!("{}", opts.short_usage(program));
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{}", VERSION);
}

fn remove_node_modules(path: &str) {
    let walker = WalkDir::new(path).into_iter();

    // Ideally it would be great to know the top level
    // node_modules directory and stop going further.
    // Not really sure how to achieve it.
    for entry in walker.filter_entry(|e| e.file_type().is_dir()) {
        let entry = entry.unwrap();

        if is_node_modules(&entry) {
            match fs::remove_dir_all(entry.path()) {
                Ok(m) => m,
                Err(f) => println!("Error: {}", f.to_string()),
            }
        }
    }

    println!("{} {}", "Done.".blue(), "¯\\_(ツ)_/¯".yellow().bold());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "Print the help menu");
    opts.optopt("p", "path", "Path to remove node_modules from", "PATH");
    opts.optflag("v", "version", "Package version");

    let arg_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        // What are the choices here? I don't want to use panic!
        Err(f) => panic!(f.to_string())
    };

    if arg_matches.opt_present("v") {
        print_version();
        return;
    }

    if arg_matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let option = arg_matches.opt_str("p");

    match option {
        Some(path) => remove_node_modules(&path),
        None => print_short_usage(&program, opts),
    };
}
