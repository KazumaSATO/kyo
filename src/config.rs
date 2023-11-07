use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    commands: Commands,
}

#[derive(Deserialize)]
struct Commands {
    sleep: Option<String>,
    poweroff: Option<String>,
    lock: Option<String>,
}

pub fn load_config(option: Option<&Path>) -> Config {
    let toml_str = match option {
        Some(path) => read_file(&path),
        None => {
            let home = std::env::var("HOME").expect("HOME was not set.");
            let path = Path::new(&home)
                .join(".config")
                .join("kanami")
                .join("config.toml");
            read_file(&path)
        }
    };
    let config: Config = toml::from_str(toml_str.as_str()).unwrap();
    config
}
fn read_file(path: &Path) -> String {
    let error_message = format!("Failed to read {}.", path.display());
    let file = File::open(path).expect(error_message.as_str());
    let mut reader = BufReader::new(file);
    let mut dest = String::from("");
    reader
        .read_to_string(&mut dest)
        .expect(error_message.as_str());
    dest
}
