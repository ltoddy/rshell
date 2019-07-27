use std::io;

use crate::cargo::Cargo;

#[derive(Clone, Debug, Default)]
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

    pub fn insert(&mut self, mut input: String) {
        input.insert(0, '\t');
        input.push('\n');
        self.body.insert(self.cursor, input);
        self.cursor += 1;
    }

    #[inline]
    pub fn prepare_playground(&self) -> io::Result<()> {
        self.cargo.new()
    }

    fn _reset(&mut self) {
        self.prepare_playground()
            .expect("Error while resetting repl");
        *self = Self::new();
    }

    pub fn show(&self) -> String {
        format!("Current Repl Code:\n{}", self.body.clone().join(""))
    }

    pub fn eval(&mut self, input: String) -> (String, String) {
        let eval_statement = format!("println!(\"{{:?}}\", {{\n{}\n}});", input);
        let mut repl = self.clone();
        repl.insert(eval_statement);

        let code = repl.body.join("");
        self.cargo
            .run(code)
            .expect("Error while evaluation expression")
    }
}
