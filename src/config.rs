use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Commands {
    sleep: Vec<String>,
    poweroff: Vec<String>,
    lock: Vec<String>,
}

pub fn load_config(option: Option<&Path>) -> Commands {
    let toml_str = match option {
        Some(path) => read_file(&path),
        None => {
            let home = std::env::var("HOME").expect("HOME was not set.");
            let path = Path::new(&home)
                .join(".config")
                .join("kanami")
                .join("config.toml");
            if path.exists() {
                read_file(&path)
            } else {
                let shared_config = Path::new("/etc/kanami/config.toml");
                read_file(&shared_config)
            }
        }
    };
    let config: ConfigFile = toml::from_str(toml_str.as_str()).unwrap();
    interpret_commands(&config)
}

#[derive(Deserialize)]
struct ConfigFile {
    commands: ConfigFileCommands,
}

#[derive(Deserialize)]
struct ConfigFileCommands {
    sleep: Option<String>,
    poweroff: Option<String>,
    lock: Option<String>,
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

fn interpret_commands(config: &ConfigFile) -> Commands {
    let truncate = |default: Vec<&str>, txt: &Option<String>| -> Vec<String> {
        match txt {
            Some(a) => a
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| String::from(x))
                .collect(),
            None => default.iter().map(|&x| String::from(x)).collect(),
        }
    };
    let commands = &config.commands;
    Commands {
        sleep: truncate(vec!["loginctl", "suspend"], &commands.sleep),
        poweroff: truncate(vec!["loginctl", "poweroff"], &commands.poweroff),
        lock: truncate(vec!["swaylock"], &commands.lock),
    }
}

#[test]
fn test_parse_config_file() {
    let config = ConfigFile {
        commands: ConfigFileCommands {
            sleep: Some(String::from("loginctl   suspend")),
            poweroff: Some(String::from("loginctl poweroff")),
            lock: Some(String::from("swaylock")),
        },
    };

    let res = interpret_commands(&config);

    assert_eq!(
        Commands {
            sleep: vec![String::from("loginctl"), String::from("suspend")],
            poweroff: vec![String::from("loginctl"), String::from("poweroff")],
            lock: vec![String::from("swaylock")],
        },
        res,
        "The consecutive spaces must be removed."
    );
}
