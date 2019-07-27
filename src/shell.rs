use std::io;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

use crate::repl::Repl;

const IN: &str = "In: ";
const OUT: &str = "Out: ";

pub struct Shell {
    buffer: String,
    repl: Repl,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            repl: Repl::new(),
        }
    }

    fn prepare(&self) -> io::Result<()> {
        self.repl.prepare_playground()?;

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.prepare()?;
        let mut reader = BufReader::new(stdin());

        loop {
            self.buffer.clear();
            print!("{}", IN);
            stdout().flush()?;
            reader.read_line(&mut self.buffer)?;

            if self.buffer.ends_with(";\n") {
                self.repl.insert(self.buffer.drain(..).collect());
            } else {
                let output = self.repl.eval(self.buffer.drain(..).collect());
                println!("{}{}", OUT, output);
            }
        }
    }
}
