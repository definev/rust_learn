use minigrep::io::Config;
use minigrep::run;
use std::process;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem is: {}", err);
        process::exit(1);
    });

    println!("Search: {}", config.query);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
