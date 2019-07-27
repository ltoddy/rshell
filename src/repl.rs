use crate::cargo::Cargo;

pub struct Repl {
    body: Vec<String>,
    cursor: usize,
    cargo: Cargo,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            body: vec!["fn main() {\n".to_string(), "}".to_string()],
            cursor: 1,
            cargo: Default::default(),
        }
    }
}
