
## Prerequisites

The Prerequisites for this project are:
  - Rust (cargo)
  - Git
  - Python 3.8 or higher
  - Python Pip 
  - Python Venv

## Installing Rust

For installing Rust:
  - visit the official website https://rust-lang.org/tools/install
  - Run the command posted there
  - Restart your terminal

You can check to see if Rust is installed by running `cargo --version` which should have an output that looks like:
```
cargo 1.76.0 (xxxxxx 2024-xxxx)
```

## MacOS or Linux Installation 

Once you have Rust cargo installed, you can install the project by running the following commands:

```
cargo install stourney
```

This will install the stourney binary in your `~/.cargo/bin` directory. You can add this to your path by running the following command:

```
export PATH=$PATH:~/.cargo/bin
```

## Windows Installation

Once you have Rust cargo installed, you can install the project by running the following commands:

```
cargo install stourney
```

This will install the stourney binary in your `~/.cargo/bin` directory. You can add this to your path by running the following command:

```
set PATH=%PATH%;~/.cargo/bin
```

## Creating a new project

Execute the following command to create a new project:

```
stourney new <project_name>
```

And follow the instructions on the screen.


## Coming Soon

- [x] Add support for creating new projects
- [ ] Add support for running projects against each other
- [ ] Add support for saving configurations and editing them
