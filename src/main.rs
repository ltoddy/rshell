use std::io::{stdin, stdout};

use crate::shell::Shell;

fn main() {
    let mut shell = Shell::new(stdin(), stdout());
    shell.prepare().expect("Error while prepare rshell");
    shell.run_forever().expect("Error while starting rshell");
}

pub mod cargo;
pub mod command;
pub mod error;
pub mod repl;
pub mod shell;
