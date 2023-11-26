use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Command as Cmd;
#[derive(Debug)]
pub struct Config {
    sleep: Command,
    poweroff: Command,
    lock: Command,
}
#[derive(Debug)]
struct Command {
    command: String,
    //icon: Option<String>,
}

impl ConfigFile {
    fn none() -> ConfigFile {
        ConfigFile {
            sleep: None,
            lock: None,
            poweroff: None,
        }
    }
}

pub fn load_config(option: Option<&Path>) -> Config {
    let toml_str = match option {
        Some(path) => Some(read_file(&path)),
        None => {
            let home = std::env::var("HOME").expect("HOME was not set.");
            let path = Path::new(&home).join(".config/kanami/config.toml");
            if path.exists() {
                Some(read_file(&path))
            } else {
                let shared_config = Path::new("/etc/kanami/config.toml");
                if shared_config.exists() {
                    Some(read_file(&shared_config))
                } else {
                    None
                }
            }
        }
    };
    let config = match toml_str {
        Some(s) => toml::from_str(s.as_str()).unwrap(),
        None => ConfigFile::none(),
    };
    interpret_config(config)
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    sleep: Option<RawCommand>,
    lock: Option<RawCommand>,
    poweroff: Option<RawCommand>,
}

#[derive(Deserialize, Debug)]
struct RawCommand {
    command: Option<String>,
    //    icon: Option<String>,
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
            //      icon: raw_command.icon,
        },
        None => Command {
            command: String::from(default_command),
            //    icon: None,
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
    pub fn run_sleep(&self) {
        self.run(&self.sleep.command)
    }
    pub fn run_poweroff(&self) {
        self.run(&self.poweroff.command)
    }

    fn run(&self, command: &String) {
        Cmd::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
    }
}
