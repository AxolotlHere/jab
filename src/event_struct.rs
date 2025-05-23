use std::env;
use std::{path::Path, process::Command};

use whoami;

pub enum Cmd<'a> {
    Whoami,
    Cd(&'a str),
    Export(String, String),
    Say(String),
    Exec(String),
    Nuke(String),
    Other(String, Vec<String>),
}

pub fn execute(command: &Cmd) -> String {
    match command {
        Cmd::Whoami => whoami::username(),
        Cmd::Cd(_path) => {
            let status = env::set_current_dir(Path::new(_path));
            let ret: String = match status {
                Ok(cmd) => env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                Err(e) => format!("throwback").to_string(),
            };

            ret
        }
        Cmd::Export(env_var, env_val) => {
            unsafe { env::set_var(env_var, env_val) };
            "".to_string()
        }
        Cmd::Nuke(env_var) => {
            unsafe {
                env::remove_var(env_var);
            }
            "".to_string()
        }
        Cmd::Say(env_var) => {
            let val = env::var(env_var);
            let ret: String = match val {
                Ok(v) => {
                    if v.to_string().contains(".sh") {
                        let mut arg_vec: Vec<String> = Vec::new();
                        arg_vec.push(v);
                        execute(&Cmd::Other("sh".to_string(), arg_vec))
                    } else {
                        let mut split_ref = v.split(" ");
                        let cmd: String = split_ref.next().unwrap().to_string();
                        let mut argv: Vec<String> = Vec::new();
                        for i in split_ref {
                            argv.push(i.to_string());
                        }
                        execute(&Cmd::Other(cmd, argv))
                    }
                }
                Err(e) => "say: Unexpected environment variable found\n".to_string(),
            };
            ret
        }
        Cmd::Exec(arg) => {
            let a = Command::new("sh").arg(arg).output();
            let res: String = match a {
                Ok(msg) => format!("{}", String::from_utf8_lossy(&msg.stdout)).to_string(),
                Err(msg) => format!("{}\n", msg).to_string(),
            };
            res
        }

        Cmd::Other(cmd, args) => {
            let a = Command::new(format!("{}", cmd)).args(args).output();
            let res: String = match a {
                Ok(msg) => format!(
                    "{}\n\x1b[34m{}\x1b[0m",
                    String::from_utf8_lossy(&msg.stdout),
                    String::from_utf8_lossy(&msg.stderr)
                )
                .to_string(),
                Err(e) => format!("{}\n", e).to_string(),
            };
            res
        }
    }
}
