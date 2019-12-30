use js_engine::exec;
use std::{env, fs::read_to_string};

pub fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let read_file;

    match args.len() {
        // No arguments passed, default to "test.js"
        1 => {
            read_file = "tests/js/test.js";
        }
        // One argument passed, assumed this is the test file
        2 => {
            read_file = &args[1];
        }
        // Some other number of arguments passed: not supported
        _ => {
            print_usage()
        }
    }

    let buffer = read_to_string(read_file)?;
    exec(&buffer);
}

fn print_usage() {
    println!(
        "Usage:
    js_engine [file.js]
    Interpret and execute file.js
    (if no file given, defaults to tests/js/test.js"
    );
}
