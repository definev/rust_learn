use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        get_param()
    }
}

fn get_param() -> Result<Config, &'static str> {
    let params: Vec<String> = env::args().collect();

    if params.len() < 3 {
        return Err("This is not enough parameters.");
    }

    let query = params[1].clone();
    let filename = params[2].clone();

    Ok(Config { query, filename })
}
