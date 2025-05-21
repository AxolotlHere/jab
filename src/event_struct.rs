use whoami;

pub enum Cmd {
    Whoami,
}

pub fn execute(command: &Cmd) -> String {
    match command {
        Cmd::Whoami => String::from(whoami::username()),
    }
}
