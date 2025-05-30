use event_struct::Cmd;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write, prelude::*};
use std::path::Path;
use std::process::exit;
use std::{env, io};

mod event_struct;

pub fn write_history(cmd: String) {
    let user = whoami::username();
    let path_ = format!("/home/{user}/.config/jab/history.json");
    let history_record_path = Path::new(&path_);
    let f_handler =
        File::open(history_record_path).expect("history: Unable to find the history for parsing");
    let reader_ = BufReader::new(&f_handler);
    let history_data: event_struct::HistoryWrapper =
        serde_json::from_reader(reader_).expect("history: invalid parsing results found");
    let write_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(history_record_path)
        .expect("history: Unable to open history file for writing");
    let mut writer_ = BufWriter::new(&write_file);
    let mut hist_data: Vec<String> = history_data.history;
    hist_data.push(cmd);
    let mut mapped_data: HashMap<String, Vec<String>> = HashMap::new();
    mapped_data.insert("history".to_string(), hist_data);
    serde_json::to_writer(&mut writer_, &mapped_data).unwrap();
    writer_.flush();
}

fn main() {
    let mut _path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap(); //Default path
    let std_reader = io::stdin();
    let std_writer = io::stdout();
    let mut job_arr: Vec<String> = Vec::new();
    let mut std_writer = BufWriter::new(std_writer);
    write!(std_writer, "\x1b[96m{_path} > \x1b[0m").unwrap();
    std_writer.flush().unwrap();
    let cmd = BufReader::new(&std_reader);
    for i in cmd.lines() {
        match i {
            Ok(inp_str) => match inp_str.trim().split(" ").next().unwrap_or("").trim() {
                "cd" => {
                    let mut part_ref: _ = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let path_ref: String = String::from(part_ref.next().unwrap().trim());
                    let cd_ret: String = event_struct::execute(&event_struct::Cmd::Cd(&path_ref));
                    if cd_ret == "throwback" {
                        write!(std_writer, "cd : No such file or directory").unwrap()
                    } else {
                        _path = env::current_dir()
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap();
                    }
                    write_history(inp_str);
                }
                "echo" => {}
                "exec" => {
                    let mut cmd = inp_str.trim().split(" ");
                    cmd.next().unwrap();
                    let sh_script = cmd.next().unwrap();
                    let res = event_struct::execute(&Cmd::Exec(sh_script.to_string()));
                    write!(std_writer, "{res}").unwrap();
                    write_history(inp_str);
                }

                "export" => {
                    let mut part_ref: _ = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let mut export_stuff = String::new();
                    for i in part_ref {
                        export_stuff += &format!("{} ", i);
                    }
                    let mut export_arg = export_stuff.trim().split("=");

                    let env_var = export_arg.next().unwrap().to_string();
                    let env_val = export_arg.next().unwrap().to_string();
                    event_struct::execute(&event_struct::Cmd::Export(env_var, env_val));
                    write_history(inp_str);
                }
                "nuke" => {
                    let mut part_ref: _ = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let env_var = part_ref.next().unwrap().to_string();
                    event_struct::execute(&event_struct::Cmd::Nuke(env_var));
                    write_history(inp_str);
                }
                "say" => {
                    let mut part_ref = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let env_var = part_ref.next().unwrap();
                    let res: String =
                        event_struct::execute(&event_struct::Cmd::Say(env_var.to_string()));
                    write_history(inp_str);
                    write!(std_writer, "{res}").unwrap()
                }
                "pwd" => {
                    write_history(inp_str);
                    write!(std_writer, "{_path}/").unwrap()
                }
                "whoami" => {
                    let ret: String = event_struct::execute(&event_struct::Cmd::Whoami);
                    write_history(inp_str);
                    writeln!(std_writer, "{ret}").unwrap()
                }
                "history" => {
                    let history_val: String = event_struct::execute(&event_struct::Cmd::History);

                    writeln!(std_writer, "{history_val}").unwrap()
                }
                "exit" => {
                    println!("Gracefully shutting down\n[Exit code : 0]");
                    exit(0);
                }
                other => {
                    let mut cmd_space = inp_str.trim().split(" ");
                    let cmd: String = cmd_space.next().unwrap().to_string();
                    let mut arg: Vec<String> = Vec::new();
                    for i in cmd_space {
                        arg.push(i.to_string());
                    }
                    let ret: String =
                        event_struct::execute(&event_struct::Cmd::Other(cmd, arg, &mut job_arr));
                    if !(inp_str.is_empty()) {
                        write_history(inp_str)
                    };
                    write!(std_writer, "{ret}").unwrap()
                }
            },
            Err(_) => {
                panic!("Unexpected Error occured: Exiting the instance");
            }
        }
        write!(std_writer, "\x1b[96m{_path} > \x1b[0m").unwrap();
        std_writer.flush().unwrap();
    }
}
