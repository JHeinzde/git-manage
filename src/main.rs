mod gitlab_client;

use std::error::Error;
use std::fs::File;
use std::io::Read;
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

    let config = config_res.unwrap();
    println!("{:?}", config);


    println!("Hello, world!");
}


fn read_config(config_name: &str) -> Result<Config, Box<dyn Error>> {
    let mut config_path = std::env::var("HOME")?;
    config_path.push_str(&"/");
    config_path.push_str(config_name);

    let mut file = File::open(config_path)?;
    let metadata = file.metadata()?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    if check_mode(mode) {
        // TODO: Replace with custom error
        panic!("The mode of the file is to permissive. Please make sure only the user can read the config file, because it contains secrets!")
    }

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = serde_json::from_str(&content)?;

    return Result::Ok(config);
}


fn check_mode(mode: u32) -> bool {
    let perm_byte = mode.to_ne_bytes()[0];

    return if perm_byte > 0o200 {
        true
    } else {
        false
    };
}
