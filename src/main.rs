pub mod cargo;
pub mod repl;

use crate::repl::Repl;

fn main() {
    let repl = Repl::new();
}
