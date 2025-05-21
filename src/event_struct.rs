struct cmd_line {
    _path_to_dir: String,
    _command: Cmd,
}

enum Cmd {
    cd(String),
    ls,
    pwd,
    echo(String),
}
