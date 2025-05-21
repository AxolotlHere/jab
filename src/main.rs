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
            Ok(inp_str) => match inp_str.as_str().trim() {
                "cd" => writeln!(std_writer, "Entered command is cd").unwrap(),
                "ls" => writeln!(std_writer, "WUT").unwrap(),
                "pwd" => writeln!(std_writer, "{_path}/").unwrap(),
                "whoami" => {
                    let ret: String = event_struct::execute(&event_struct::Cmd::Whoami);
                    writeln!(std_writer, "{ret}").unwrap()
                }
                "exit" => {
                    println!("Gracefully shutting down\n[Exit code : 0]");
                    exit(0);
                }
                other => writeln!(std_writer, "Err : Unknown command found").unwrap(),
            },
            Err(_) => {
                panic!("Unexpected Error occured: Exiting the instance");
            }
        }
        write!(std_writer, "{_path} > ").unwrap();
        std_writer.flush().unwrap();
    }
}
