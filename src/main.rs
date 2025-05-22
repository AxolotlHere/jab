use std::fmt::write;
use std::io;
use std::io::{BufReader, BufWriter, Write, prelude::*};
use std::process::exit;

use event_struct::Cmd;
use whoami;
mod event_struct;

fn main() {
    let mut _path: String = String::from("~"); //Default path
    let std_reader = io::stdin();
    let std_writer = io::stdout();
    let mut std_writer = BufWriter::new(std_writer);
    write!(std_writer, "{_path} > ").unwrap();
    std_writer.flush().unwrap();
    let cmd = BufReader::new(&std_reader);
    for i in cmd.lines() {
        match i {
            Ok(inp_str) => match inp_str.trim().split(" ").next().unwrap_or("").trim() {
                "cd" => write!(std_writer, "Entered command is cd").unwrap(),
                "pwd" => write!(std_writer, "{_path}/").unwrap(),
                "whoami" => {
                    let ret: String = event_struct::execute(&event_struct::Cmd::Whoami);
                    write!(std_writer, "{ret}").unwrap()
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
        write!(std_writer, "{_path} > ").unwrap();
        std_writer.flush().unwrap();
    }
}
