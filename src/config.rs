use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::{Command as Cmd, Output};

#[derive(PartialEq, Debug)]
pub struct Config {
    sleep: Command,
    poweroff: Command,
    lock: Command,
}

#[derive(PartialEq, Debug)]
pub struct Command {
    command: String,
    icon: Option<String>,
}

pub fn load_config(option: Option<&Path>) -> Config {
    let toml_str = match option {
        Some(path) => read_file(&path),
        None => {
            let home = std::env::var("HOME").expect("HOME was not set.");
            let path = Path::new(&home).join(".config/kanami/config.toml");
            if path.exists() {
                read_file(&path)
            } else {
                let shared_config = Path::new("/etc/kanami/config.toml");
                read_file(&shared_config)
            }
        }
    };
    let config: ConfigFile = toml::from_str(toml_str.as_str()).unwrap();
    interpret_config(config)
}

#[derive(Deserialize)]
struct ConfigFile {
    sleep: Option<RawCommand>,
    lock: Option<RawCommand>,
    poweroff: Option<RawCommand>,
}

#[derive(Deserialize)]
struct RawCommand {
    command: Option<String>,
    icon: Option<String>,
}

/// Read a text file at `path`.
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

fn as_command(default_command: &str, raw: Option<RawCommand>) -> Command {
    match raw {
        Some(raw_command) => Command {
            command: raw_command.command.unwrap_or(String::from(default_command)),
            icon: raw_command.icon,
        },
        None => Command {
            command: String::from(default_command),
            icon: None,
        },
    }
}

fn interpret_config(config: ConfigFile) -> Config {
    Config {
        sleep: as_command("loginctl suspend", config.sleep),
        lock: as_command("swaylock", config.lock),
        poweroff: as_command("loginctl poweroff", config.poweroff),
    }
}

impl Config {
    pub fn run_lock(&self) {
        self.run(&self.lock.command)
    }
    pub fn run_sleep(&self) {}
    pub fn run_poweroff(&self) {}

    fn run(&self, command: &String) {
        Cmd::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
    }
}

// #[test]
// fn test_parse_config_file() {
//     let config = ConfigFile {
//         commands: ConfigFileCommands {
//             sleep: Some(String::from("loginctl   suspend")),
//             poweroff: Some(String::from("loginctl poweroff")),
//             lock: Some(String::from("swaylock")),
//         },
//     };

//     let res = interpret_commands(&config);

//     assert_eq!(
//         Commands {
//             sleep: vec![String::from("loginctl"), String::from("suspend")],
//             poweroff: vec![String::from("loginctl"), String::from("poweroff")],
//             lock: vec![String::from("swaylock")],
//         },
//         res,
//         "The consecutive spaces must be removed."
//     );
// }
