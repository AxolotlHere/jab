use std::fmt::write;
use std::io;
use std::io::{BufReader, BufWriter, Write, prelude::*};

fn main() {
    let mut _path: String = String::from("~"); //Default path
    let std_reader = io::stdin();
    let std_writer = io::stdout();
    let mut std_writer = BufWriter::new(std_writer);
    write!(std_writer, "{_path} > ").unwrap();
    std_writer.flush().unwrap();
    let cmd = BufReader::new(std_reader);
    for i in cmd.lines() {
        match i {
            Ok(inp_str) => match inp_str.as_str() {
                "cd" => println!("Entered command is cd"),
                "ls" => println!("WUT"),
                other => println!("Err : Unknown command found"),
            },
            Err(_) => {
                panic!("Unexpected Error occured: Exiting the instance");
            }
        }
        write!(std_writer, "{_path} > ").unwrap();
        std_writer.flush().unwrap();
    }
}
