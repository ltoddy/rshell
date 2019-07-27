pub mod cargo;
pub mod repl;
pub mod shell;

use crate::shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run().expect("Error while starting rshell");
}
