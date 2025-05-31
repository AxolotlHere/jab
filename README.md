# jab

**jab** (Just Another Bash) is a lightweight shell implemented in Rust. It aims to provide a minimalist yet extendable command-line interface, supporting essential features such as command execution, background jobs, and persistent history.

## Features

- Basic command execution (`cd`, `pwd`, `whoami`, etc.)
- Background process support (`&` and `jobs`)
- Environment variable manipulation (`export`, `nuke`, `say`)
- Persistent history saved in `~/.config/jab/history.json`
- Simple scripting hooks and custom commands (`exec`, `say`)
- Clean internal architecture using Rust enums for commands

## UPDATES

Currently working on pipelines and output redirection, wouldnt be going public until proper testing

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- Unix-like environment (Linux, macOS)

### Installation

```bash
git clone https://github.com/your-username/jab.git
cd jab
cargo build --release
```

run

```
mkdir -p ~/.config/jab
touch ~/.config/jab/history.json
```

## ADD Jab to path

```
sudo ln -s $(pwd)/target/release/jab /usr/local/bin/jab
```

Contributions are welcome. Please open issues or pull requests with clear descriptions.

    >>>Fork the repository
    >>>Create your feature branch (git checkout -b feature/your-feature)
    >>>Commit your changes (git commit -am 'Add new feature')
    >>>Push to the branch (git push origin feature/your-feature)
    >>>Open a pull request
