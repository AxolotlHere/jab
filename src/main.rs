use std::io::{BufReader, BufWriter, Write, prelude::*};
use std::process::exit;
use std::{env, io};

use event_struct::Cmd;
use whoami;
mod event_struct;

fn main() {
    let mut _path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap(); //Default path
    let std_reader = io::stdin();
    let std_writer = io::stdout();
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
                }
                "echo" => {}
                "exec" => {
                    let mut cmd = inp_str.trim().split(" ");
                    cmd.next().unwrap();
                    let sh_script = cmd.next().unwrap();
                    let res = event_struct::execute(&Cmd::Exec(sh_script.to_string()));
                    write!(std_writer, "{res}").unwrap();
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
                }
                "nuke" => {
                    let mut part_ref: _ = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let env_var = part_ref.next().unwrap().to_string();
                    event_struct::execute(&event_struct::Cmd::Nuke(env_var));
                }
                "say" => {
                    let mut part_ref = inp_str.trim().split(" ");
                    let cmd = part_ref.next().unwrap();
                    let env_var = part_ref.next().unwrap();
                    let res: String =
                        event_struct::execute(&event_struct::Cmd::Say(env_var.to_string()));
                    write!(std_writer, "{res}").unwrap()
                }
                "pwd" => write!(std_writer, "{_path}/").unwrap(),
                "whoami" => {
                    let ret: String = event_struct::execute(&event_struct::Cmd::Whoami);
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
                    let ret: String = event_struct::execute(&event_struct::Cmd::Other(cmd, arg));
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
