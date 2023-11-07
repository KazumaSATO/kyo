use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    commands: Commands,
}

#[derive(Deserialize)]
struct Commands {
    sleep: Option<String>,
    poweroff: Option<String>,
    lock: Option<String>,
}

pub fn load_config(option: &Option<String>) {}
