use std::io::{stdin, stdout, BufRead, BufReader, Stdin, Write};
use std::process;

use crate::error::Result;
use crate::repl::Repl;

#[derive(Debug, Default)]
pub struct Shell {
    buffer: String,
    repl: Repl,
}

impl Shell {
    const IN: &'static str = "In: ";
    const OUT: &'static str = "Out: ";

    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            repl: Repl::new(),
        }
    }

    pub fn prepare(&self) -> Result<()> {
        self.repl.prepare_playground()?;

        Ok(())
    }

    fn read(&mut self, reader: &mut BufReader<Stdin>) -> Result<String> {
        print!("{}", Self::IN);
        stdout().flush()?;

        let mut buffer = String::with_capacity(64);
        reader.read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut reader = BufReader::new(stdin());

        loop {
            self.buffer = self.read(&mut reader)?;

            if self.buffer.is_empty() {
                continue;
            }

            if self.buffer.starts_with(':') {
                self.dispatch_builtin_command(&self.buffer);
            } else if self.buffer.ends_with(';') {
                self.repl.insert(self.buffer.drain(..).collect());
            } else {
                let (stdout_output, _stderr_output) =
                    self.repl.eval(self.buffer.drain(..).collect());
                println!("{}{}", Self::OUT, stdout_output);
            }
        }
    }

    fn dispatch_builtin_command(&self, command: &str) {
        match command {
            ":quit" | ":exit" => self.exit(),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn exit(&self) {
        process::exit(0);
    }
}
