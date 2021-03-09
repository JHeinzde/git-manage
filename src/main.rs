use std::error::Error;
use std::fs::File;
use std::io;
use std::os::unix::fs::PermissionsExt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    api_token: String,
    url: String,
    proxy: String,
}


fn main() {
    let config_res = read_config(".helper_config.json");
    if config_res.is_err() {
        println!("{}", config_res.unwrap_err());
        panic!("Error while reading config");
    }

    println!("Hello, world!");
}


fn read_config(config_name: &str) -> Result<Config, Box<dyn Error>> {
    let mut config_path = std::env::var("HOME")?;
    config_path.push_str(&"/");
    config_path.push_str(config_name);

    println!("{}", config_path);

    let file = File::open(config_path)?;
    let metadata = file.metadata()?;

    let permissions = metadata.permissions();
    let mode = permissions.mode();
    if !check_mode(mode) {
        // TODO: Replace with custom error
        panic!("The mode of the file is to permissive. Please make sure only the user can read the config file, because it contains secrets!")
    }

    return Result::Ok(Config {
        api_token: String::new(),
        url: String::new(),
        proxy: String::new(),
    });
}


fn check_mode(mode: u32) -> bool {
    println!("{:?}", mode.to_be_bytes());
    for b in mode.to_ne_bytes().iter() {
        println!("{:o}", b)
    }

    let perm_byte = mode.to_ne_bytes()[0];

    return if perm_byte > 0o200 {
        true
    } else {
        false
    };
}
