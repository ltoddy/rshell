use std::fmt::Display;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process;

use crate::command::BuiltinCommand;
use crate::error::{RShellError, Result};
use crate::repl::Repl;

#[derive(Debug)]
pub struct Shell<R, W: Write> {
    buffer: String,
    repl: Repl,

    cin: BufReader<R>,
    cout: BufWriter<W>,
}

impl<R: Read, W: Write> Shell<R, W> {
    const IN: &'static str = "In: ";
    const OUT: &'static str = "Out: ";

    pub fn new(cin: R, cout: W) -> Self {
        Self {
            buffer: String::with_capacity(1024),
            repl: Repl::new(),

            cin: BufReader::new(cin),
            cout: BufWriter::new(cout),
        }
    }

    pub fn prepare(&self) -> Result<()> {
        self.repl.prepare_playground()?;

        Ok(())
    }

    fn read(&mut self) -> Result<String> {
        self.cout.write_all(Self::IN.as_bytes())?;
        self.cout.flush()?;

        let mut buffer = String::with_capacity(64);
        self.cin.read_line(&mut buffer)?;

        Ok(buffer.trim().to_string())
    }

    #[allow(dead_code)] // TODO: remove
    fn write<T: Display>(&mut self, content: T) -> Result<()> {
        write!(self.cout, "{}{}", Self::OUT, content)?;
        Ok(())
    }

    fn writeln<T: Display>(&mut self, content: T) -> Result<()> {
        writeln!(self.cout, "{}{}", Self::OUT, content)?;
        Ok(())
    }

    pub fn run_forever(&mut self) -> Result<()> {
        loop {
            self.buffer = self.read()?;

            if self.buffer.is_empty() {
                continue;
            }

            if self.buffer.starts_with(':') {
                if let Err(e) = self.dispatch_builtin_command() {
                    eprintln!("{}", e);
                    continue;
                }
            } else if self.buffer.ends_with(';') {
                self.repl.insert(self.buffer.drain(..).collect());
            } else {
                let (stdout_output, _stderr_output) = self.repl.eval(self.buffer.drain(..).collect());
                self.writeln(stdout_output)?;
            }
        }
    }

    fn dispatch_builtin_command(&mut self) -> Result<()> {
        match BuiltinCommand::from(self.buffer.clone()) {
            BuiltinCommand::Exit => self.exit(),
            BuiltinCommand::ShowCode => self.show(),
            BuiltinCommand::Clear => self.clear(),
            BuiltinCommand::Invalid(input) => return Err(RShellError::InvalidBuiltinCommand(input)),
        }

        Ok(())
    }

    #[inline]
    fn exit(&self) {
        process::exit(0);
    }

    fn show(&self) {
        let code = self.repl.show();
        println!("{}", code);
    }

    #[inline]
    fn clear(&mut self) {
        self.repl.clear();
    }
}
