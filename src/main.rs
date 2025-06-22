mod atcoder_client;
mod html_parser;
mod judger;

use anyhow::Result;
use atcoder_client::AtcoderClient;
use clap::Parser;
use html_parser::HtmlParser;
use judger::Judger;

#[derive(Parser)]
#[command(
    name = "cargo-acj",
    about = "AtCoderのサンプルテストを自動実行するコマンド",
    long_about = None
)]
struct Cli {
    #[arg(help = "ex: abc123, typical90, ...")]
    contest_name: String,
    #[arg(help = "url suffix. ex: a, ax, ...")]
    problem: String,
    #[arg(long, help = "TLE[sec]. デフォルト2.0秒")]
    tle: Option<f64>,
    #[arg(long, help = "実行bin. デフォルトはmain")]
    bin: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse_from(std::env::args_os().skip(1));

    let client = AtcoderClient::new(cli.contest_name.as_str(), cli.problem.as_str());
    let html = client.get_html()?;

    let parser = HtmlParser::new(html.as_str());
    let samples = parser.get_sample()?;

    let judger = Judger::set_up(samples, cli.bin, cli.tle)?;
    judger.run()?;
    Ok(())
}
