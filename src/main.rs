use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;

use serde::{Deserialize, Serialize};

mod gitlab_client;

fn main() {
    let config_res = read_config(".helper_config.json");
    if config_res.is_err() {
        println!("{}", config_res.unwrap_err());
        panic!("Error while reading config");
    }

    let git_lab = config_res.unwrap();

    let p_branches = git_lab.get_protected_branches_project("31");
    if p_branches.is_err() {
        panic!("Could not get protected branches!");
    }

    let p_branches = p_branches.unwrap();
}


fn read_config(config_name: &str) -> Result<gitlab_client::GitLab, Box<dyn Error>> {
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
    let config: gitlab_client::GitLab = serde_json::from_str(&content)?;

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
