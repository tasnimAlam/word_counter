extern crate word_counter;

use std::process;
use structopt::StructOpt;
use word_counter::Opt;

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = word_counter::run(opt) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
