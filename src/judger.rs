use colored::Colorize;
use itertools::{Itertools, izip};
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

    const EPS: f64 = 1e-10;

    fn tokens_equal(a: &str, b: &str) -> bool {
        if a == b {
            return true;
        }
        let (fa, fb) = (a.parse::<f64>(), b.parse::<f64>());
        match (fa, fb) {
            (Ok(da), Ok(db)) => {
                let diff = (da - db).abs();
                diff < Self::EPS
            }
            _ => false,
        }
    }

    fn print_ok(problem_idx: usize) {
        println!(
            "{}",
            format!("Sample {} Passed!!", problem_idx + 1)
                .green()
                .bold()
        );
    }

    fn print_failed(input: &str, expected: &str, actual: &str, problem_idx: usize) {
        println!(
            "{}",
            format!("Sample {} failed...", problem_idx + 1).red().bold()
        );
        println!("-- Input:\n{}", input);
        println!("-- Expected:\n{}", expected);
        println!("-- Actual:\n{}", actual);
    }

    fn collect_judge(input: &str, expected: &str, actual: &str, problem_idx: usize) {
        let expected = expected.split('\n').map(|e| e.trim_end()).join("\n");
        let actual = actual.split('\n').map(|e| e.trim_end()).join("\n");
        if actual == expected.trim_end() {
            Self::print_ok(problem_idx);
        } else {
            let expected_v = expected.split('\n').collect_vec();
            let actual_v = actual.split('\n').collect_vec();
            if expected_v.len() != actual_v.len() {
                Self::print_failed(input, expected.as_str(), actual.as_str(), problem_idx);
                return;
            }
            for (expe, actu) in izip!(expected_v, actual_v) {
                if !Self::tokens_equal(expe, actu) {
                    Self::print_failed(input, expected.as_str(), actual.as_str(), problem_idx);
                    return;
                }
            }
            Self::print_ok(problem_idx);
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
