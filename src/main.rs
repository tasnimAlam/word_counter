extern crate word_counter;

use std::env;
use std::process;

use word_counter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("filename {}", config.filename);

    if let Err(e) = word_counter::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
