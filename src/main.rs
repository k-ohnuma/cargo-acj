use anyhow::Result;
use cargo_atest::{atcoder_client::AtcoderClient, html_parser::HtmlParser};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "cargo-atest",
    about = "AtCoderのサンプルテストを自動実行するコマンド",
    long_about = None
)]
struct Cli {
    #[arg(help = "ex: abc123, typical90, ...")]
    contest_name: String,
    #[arg(help = "url prefix. ex: a, ax, ...")]
    problem: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = AtcoderClient::new(cli.contest_name.as_str(), cli.problem.as_str());
    let html = client.get_html()?;
    let parser = HtmlParser::new(html.as_str());
    let samples = parser.get_sample()?;
    println!("{:?}", samples);
    Ok(())
}
