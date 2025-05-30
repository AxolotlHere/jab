use once_cell::sync::Lazy;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::Stdio;
use std::sync::Mutex;
use std::thread;
use std::{path::Path, process::Command};
use whoami;

static job_list: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
fn add_job(job_cmd: &String) {
    let mut job_lock = job_list.lock().unwrap();
    job_lock.push(job_cmd.clone());
}

pub enum Cmd<'a> {
    Whoami,
    Cd(&'a str),
    Export(String, String),
    Say(String, String),
    Exec(String),
    Var(String),
    Nuke(String),
    History,
    Job,
    Other(String, Vec<String>, String),
}

#[derive(Debug, serde::Deserialize)]
pub struct HistoryWrapper {
    pub history: Vec<String>,
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
        Cmd::Var(env_var) => {
            let val = env::var(env_var);
            let ret: String = match val {
                Ok(v) => v,
                Err(e) => "var: Mismatch in the variable".to_string(),
            };
            ret
        }
        Cmd::Say(env_var, inp_str) => {
            let val = env::var(env_var);
            let ret: String = match val {
                Ok(v) => {
                    if v.to_string().contains(".sh") {
                        let mut arg_vec: Vec<String> = Vec::new();
                        arg_vec.push(v);
                        execute(&Cmd::Other("sh".to_string(), arg_vec, inp_str.clone()))
                    } else {
                        let mut split_ref = v.split(" ");
                        let cmd: String = split_ref.next().unwrap().to_string();
                        let mut argv: Vec<String> = Vec::new();
                        for i in split_ref {
                            argv.push(i.to_string());
                        }
                        execute(&Cmd::Other(cmd, argv, inp_str.clone()))
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
        Cmd::History => {
            let mut res: String = String::new();
            let user = whoami::username();
            let path_ = format!("/home/{user}/.config/jab/history.json");
            let history_record_path = Path::new(&path_);
            let f_handler = File::open(history_record_path)
                .expect("history: Unable to find the history for parsing");
            let reader_ = BufReader::new(f_handler);
            let history_data: HistoryWrapper =
                serde_json::from_reader(reader_).expect("history: invalid parsing results found");
            res = history_data.history.join("\r\n");
            res
        }
        Cmd::Job => {
            let mut str_1: String = String::from("ID\tProcess\n");
            let mut counter_: i32 = 1;
            if job_list.lock().unwrap().is_empty() {
                str_1 = String::from("No background tasks running yet.")
            } else {
                for i in job_list.lock().unwrap().iter() {
                    str_1 += &format!("{counter_}\t{i}\n");
                    counter_ += 1;
                }
            }
            str_1
        }
        Cmd::Other(cmd, args, inp_str) => {
            if !(args.is_empty()) {
                if args.get(args.len() - 1).unwrap() != "&" {
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
                } else {
                    let a = Command::new(format!("{}", &cmd))
                        .args(&args[0..=args.len() - 2])
                        .stdin(Stdio::null())
                        .stderr(Stdio::null())
                        .stdout(Stdio::null())
                        .spawn();

                    let res: String = match a {
                        Ok(mut msg) => {
                            let inp_str_clone: String = inp_str.clone();
                            thread::spawn(move || {
                                add_job(&inp_str_clone);
                                let res = msg.wait();
                                match res {
                                    Ok(msg) => {
                                        let n = job_list.lock().iter().len();
                                        for i in 0..n {
                                            if job_list.lock().unwrap()[i] == inp_str_clone {
                                                job_list.lock().unwrap().remove(i);
                                            }
                                        }
                                    }
                                    Err(woah) => {
                                        "Bruh";
                                    }
                                }
                            });
                            "".to_string()
                        }
                        Err(msg) => "spawn error: Unable to spawn the job".to_string(),
                    };
                    res
                }
            } else {
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
}
