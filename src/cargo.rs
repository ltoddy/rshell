use std::env::temp_dir;
use std::fs::{remove_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Cargo {
    temp_dir: PathBuf,
    playground_dir: PathBuf,
    main_file: PathBuf,
}

impl Default for Cargo {
    fn default() -> Self {
        let temp_dir = temp_dir();
        let playground_dir = {
            let mut dir = temp_dir.clone();
            dir.push("playground_dir");
            dir
        };
        let main_file = {
            let mut file = playground_dir.clone();
            file.push("src/main.rs");
            file
        };

        Self {
            temp_dir,
            playground_dir,
            main_file,
        }
    }
}

macro_rules! execute_cargo_program {
    ($current_dir: expr, $($arg: expr),*) => {
        Command::new("cargo")
                .current_dir($current_dir)
                $(.arg($arg))*
                .output()
                .expect("Error while execute cargo program")
    };

    ($current_dir: expr, $($arg: expr,)*) => {
        execute_cargo_program!($current_dir, $($arg),*)
    };

    ($($arg: expr),*) => {
        execute_cargo_program!(std::env::current_dir().expect("Error can't get current dir"), $($arg),*)
    };

    ($($arg: expr,)*) => {
        execute_cargo_program!(std::env::current_dir().expect("Error can't get current dir"), $($arg),*)
    };
}

impl Cargo {
    #[allow(clippy::wrong_self_convention, clippy::new_ret_no_self)]
    pub fn new(&self) -> Result<()> {
        if Path::new(&self.playground_dir).exists() {
            remove_dir_all(&self.playground_dir)?;
        }

        execute_cargo_program!(&self.temp_dir, "new", &self.playground_dir);

        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        execute_cargo_program!(&self.playground_dir, "build");

        Ok(())
    }

    pub fn run(&self, code: String) -> Result<(String, String)> {
        println!("self  =====> : {:?}", self);
        let mut main = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.main_file)?;

        write!(main, "{}", code)?;

        let output: Output = execute_cargo_program!(&self.playground_dir, "run");

        let stdout = String::from_utf8(output.stdout).expect("Invalid input(not UTF-8)");
        let stderr = String::from_utf8(output.stderr).expect("Invalid input(not UTF-8)");

        Ok((stdout, stderr))
    }
}

#[cfg(test)]
pub mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    use super::Cargo;

    #[test]
    pub fn test_cargo_new_command_basic() {
        let cargo: Cargo = Default::default();
        cargo.new().unwrap();

        assert!(Path::exists(&cargo.temp_dir));
        assert!(Path::exists(&cargo.playground_dir));
        assert!(Path::exists(&cargo.main_file));
    }

    #[test]
    pub fn test_cargo_new_command() {
        let cargo: Cargo = Default::default();
        cargo.new().unwrap();

        let file = File::open(cargo.main_file).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = String::with_capacity(1024);
        reader.read_to_string(&mut buf).unwrap();

        assert_eq!(buf, ("fn main() {\n    println!(\"Hello, world!\");\n}\n"));
    }

    #[test]
    pub fn test_cargo_run_command() {
        let cargo: Cargo = Default::default();
        cargo.new().unwrap();
        cargo.build().unwrap();

        let code = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
        let (std_out, _std_err) = cargo.run(code.to_owned()).unwrap();

        assert_eq!("Hello, world!\n", std_out)
    }
}
