pub mod io;

use io::Config;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    
    print!("Content in file:\n{}", contents);

    Ok(())
}
