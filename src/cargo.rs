use std::env::temp_dir;
use std::ffi::OsStr;
use std::fs::{remove_dir_all, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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

impl Cargo {
    #[allow(clippy::wrong_self_convention, clippy::new_ret_no_self)]
    pub fn new(&self) -> io::Result<()> {
        if Path::new(&self.playground_dir).exists() {
            remove_dir_all(&self.playground_dir)?;
        }

        Command::new("cargo")
            .current_dir(&self.temp_dir)
            .args(&[OsStr::new("new"), self.playground_dir.as_os_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn build(&self) -> io::Result<()> {
        Command::new("cargo")
            .current_dir(&self.playground_dir)
            .arg("build")
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn run(&self, code: String) -> io::Result<String> {
        let mut main = File::create(&self.main_file)?;
        write!(main, "{}", code)?;

        let output = Command::new("cargo")
            .current_dir(&self.playground_dir)
            .arg("run")
            .output()?;

        let stdout = String::from_utf8(output.stdout).expect("Invalid input(not UTF-8)");
        let stderr = String::from_utf8(output.stderr).expect("Invalid input(not UTF-8)");

        if stdout.is_empty() {
            Ok(stderr)
        } else {
            Ok(stdout)
        }
    }
}
