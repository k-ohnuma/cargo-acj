use colored::Colorize;
use std::{
    io::Write,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

use anyhow::Result;

pub(crate) struct Judger {
    samples: Vec<(String, String)>,
    run_bin: Option<String>,
    tle_time: f64,
}

impl Judger {
    pub fn set_up(
        samples: Vec<(String, String)>,
        run_bin: Option<String>,
        tle: Option<f64>,
    ) -> Result<Self> {
        let bin = run_bin.to_owned();

        Command::new("cargo")
            .env("RUSTFLAGS", "-Awarnings")
            .args(["build", "--release", "--quiet"])
            .args(if bin.is_some() {
                vec!["--bin".to_owned(), bin.unwrap()]
            } else {
                vec![]
            })
            .status()?;

        Ok(Self {
            samples,
            run_bin,
            tle_time: tle.unwrap_or(2.0),
        })
    }

    fn collect_judge(input: &str, expected: &str, actual: &str, problem_idx: usize) {
        if actual == expected.trim_end() {
            println!(
                "{}",
                format!("Sample {} Passed!!", problem_idx + 1)
                    .green()
                    .bold()
            );
        } else {
            println!(
                "{}",
                format!("Sample {} failed...", problem_idx + 1).red().bold()
            );
            println!("-- Input:\n{}", input);
            println!("-- Expected:\n{}", expected);
            println!("-- Actual:\n{}", actual);
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("checking...");
        println!("{}", "=".repeat(50));
        for (i, (input, expected)) in self.samples.iter().enumerate() {
            let bin = self.run_bin.to_owned();

            let now = Instant::now();
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
                .stderr(Stdio::piped())
                .spawn()?;

            let stdin = run_command.stdin.as_mut().unwrap();
            stdin.write_all(input.as_bytes())?;

            let timeout = Duration::from_secs_f64(self.tle_time);

            match run_command.wait_timeout(timeout)? {
                Some(status) => {
                    let output = run_command.wait_with_output()?;
                    let time = now.elapsed().as_secs_f64();
                    if !status.success() {
                        println!("{}", format!("Sample {} failed...", i + 1).red().bold());
                        let error = String::from_utf8_lossy(&output.stderr).to_string();
                        println!("{}", error);
                    } else {
                        let actual = String::from_utf8_lossy(&output.stdout)
                            .trim_end()
                            .to_string();
                        Self::collect_judge(input, expected, &actual, i);
                    }
                    println!("time: {}[secs]", time);
                }
                None => {
                    run_command.kill()?;
                    run_command.wait()?;
                    println!("{}", format!("Sample {} failed...", i + 1).red().bold());
                    println!(
                        "{}",
                        format!("TLE: {}[secs] elapsed.", self.tle_time)
                            .red()
                            .bold()
                    );
                }
            }

            println!("{}", "=".repeat(50));
        }
        Ok(())
    }
}
