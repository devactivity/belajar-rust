// Use this file after Closure & Iterator
// Closure: https://www.youtube.com/watch?v=Dru1IjXGN9A
// Iterator: https://www.youtube.com/watch?v=-EG4S7wWJ74

use std::{env, process};

use lib_mod::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(| err | {
    // Args Iterator

    let config = Config::new(env::args()).unwrap_or_else(| err | {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = lib_mod::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
