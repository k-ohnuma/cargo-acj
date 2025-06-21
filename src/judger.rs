use colored::Colorize;
use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;

pub struct Judger {
    samples: Vec<(String, String)>,
    run_bin: Option<String>,
}

impl Judger {
    pub fn set_up(samples: Vec<(String, String)>, run_bin: Option<String>) -> Result<Self> {
        let bin = run_bin.to_owned();
        Command::new("cargo")
            .args(["build", "--release", "--quiet"])
            .args(if bin.is_some() {
                vec!["--bin".to_owned(), bin.unwrap()]
            } else {
                vec![]
            })
            .status()?;

        Ok(Self { samples, run_bin })
    }

    pub fn run(&self) -> Result<()> {
        println!("checking...");
        println!("{}", "=".repeat(50));
        for (i, (input, expected)) in self.samples.iter().enumerate() {
            let bin = self.run_bin.to_owned();
            let mut run_command = Command::new("cargo")
                .env("RUSTFLAGS", "-Awarnings")
                .args(["run", "--release", "--quiet"])
                .args(if bin.is_some() {
                    vec!["--bin".to_owned(), bin.unwrap()]
                } else {
                    vec![]
                })
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            let stdin = run_command.stdin.as_mut().unwrap();
            stdin.write_all(input.as_bytes())?;

            let output = run_command.wait_with_output()?;
            let actual = String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_string();
            if actual == expected.trim_end() {
                println!("{}", format!("Sample {} Passed!!", i + 1).green().bold());
            } else {
                println!("{}", format!("Sample {} failed...", i + 1).red().bold());
                println!("-- Input:\n{}", input);
                println!("-- Expected:\n{}", expected);
                println!("-- Actual:\n{}", actual);
            }

            println!("{}", "=".repeat(50));
        }
        Ok(())
    }
}
