use crate::config::config::{Command, Config};
use serde::Deserialize;
use std::convert::AsRef;
use std::env::var;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use toml;
/// Read a configuration file.
pub(crate) fn read_config<P: AsRef<Path>>(option: Option<P>) -> Config {
    let mut paths: Vec<PathBuf> = vec![];
    if let Some(p) = option {
        paths.push(p.as_ref().to_path_buf());
    }
    if let Ok(home) = var("HOME") {
        paths.push(Path::new(&home).join(".config/kanami/config.toml"));
    }
    paths.push(Path::new("etc").join("kanami/config.toml"));

    for path in paths {
        if path.exists() {
            let display = path.display();
            let text =
                read_to_string(&path).expect(format!("Failed to read {}.", display).as_str());
            let config_file: ConfigFile =
                toml::from_str(text.as_str()).expect(format!("Syntax error: {}", display).as_str());
            return config_file.as_config();
        }
    }
    Config::default()
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
    icon: Option<Icon>,
}

#[derive(Deserialize, Debug)]
struct Icon {
    color: Option<String>,
}

impl ConfigFile {
    fn as_config(self) -> Config {
        let base = Config::default();
        Config {
            sleep: ConfigFile::to_command(self.sleep, base.sleep),
            lock: ConfigFile::to_command(self.lock, base.lock),
            poweroff: ConfigFile::to_command(self.poweroff, base.poweroff),
        }
    }

    fn to_command(raw: Option<RawCommand>, base: Command) -> Command {
        match raw {
            Some(r) => Command {
                command: r.command.unwrap_or(base.command),
                icon: match r.icon {
                    Some(color) => match color.color {
                        Some(s) => s,
                        None => base.icon,
                    },
                    None => base.icon,
                },
            },
            None => base,
        }
    }
}
