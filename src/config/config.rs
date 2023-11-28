use std::process::Command as Cmd;
#[derive(Debug)]
pub(crate) struct Config {
    pub(super) sleep: Command,
    pub(super) poweroff: Command,
    pub(super) lock: Command,
}
#[derive(Debug)]
pub(super) struct Command {
    pub(super) command: String,
    /// The CSS color code for the command.
    pub(super) icon: String,
}

pub(crate) enum Commands {
    LOCK,
    SLEEP,
    POWEROFF,
}

impl Config {
    pub(crate) fn run(&self, command: &Commands) {
        match command {
            Commands::LOCK => self.call_shell(&self.lock.command),
            Commands::SLEEP => self.call_shell(&self.sleep.command),
            Commands::POWEROFF => self.call_shell(&self.poweroff.command),
        };
    }
    pub(crate) fn get_color(&self, command: &Commands) -> &String {
        match command {
            Commands::LOCK => &self.lock.icon,
            Commands::SLEEP => &self.sleep.icon,
            Commands::POWEROFF => &self.poweroff.icon,
        }
    }

    pub(super) fn default() -> Config {
        Config {
            sleep: Command {
                command: String::from("loginctl suspend"),
                icon: String::from("#ffffff"),
            },
            poweroff: Command {
                command: String::from("loginctl poweroff"),
                icon: String::from("#ffffff"),
            },
            lock: Command {
                command: String::from("swaylock"),
                icon: String::from("#ffffff"),
            },
        }
    }

    fn call_shell(&self, command: &String) {
        Cmd::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
    }
}
