use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    if let Ok(config) = config {
        let query = &config.query;
        let filename = &config.filename;

        println!("Searching for {query}");
        println!("In file {filename}");

        if let Err(e) = run(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
}
