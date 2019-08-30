use std::{env, process};

use changelog_writer::Config;

fn main() {
    // argument format: <major|minor> <mdPath> <htmlPath>
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Updating type {:?}", config.version_type);
    println!("path {}", config.path);
}

