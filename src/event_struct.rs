use std::process::Command;

use whoami;

pub enum Cmd {
    Whoami,
    Other(String, Vec<String>),
}

pub fn execute(command: &Cmd) -> String {
    match command {
        Cmd::Whoami => whoami::username(),

        Cmd::Other(cmd, args) => {
            let a = Command::new(format!("{}", cmd)).args(args).output();
            let res: String = match a {
                Ok(msg) => format!(
                    "{}\n\x1b[34m{}\x1b[0m",
                    String::from_utf8_lossy(&msg.stdout),
                    String::from_utf8_lossy(&msg.stderr)
                )
                .to_string(),
                Err(e) => format!("{}", e).to_string(),
            };
            res
        }
    }
}
