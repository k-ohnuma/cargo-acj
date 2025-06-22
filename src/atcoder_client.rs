use anyhow::{Context, Result, anyhow};
use reqwest::Url;

pub const ATCODER_BASE_URL: &str = "https://atcoder.jp/contests";

pub(crate) struct AtcoderClient {
    pub endpoint: Url,
    pub client: reqwest::blocking::Client,
}

impl AtcoderClient {
    pub fn new(contest_name: &str, problem: &str) -> Result<Self> {
        let endpoint = format!("{ATCODER_BASE_URL}/{contest_name}/tasks/{contest_name}_{problem}");
        let endpoint = Url::parse(&endpoint).context(format!("invalid url: {}", endpoint))?;
        Ok(Self {
            endpoint,
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn get_html(&self) -> Result<String> {
        let resp = self.client.get(self.endpoint.to_owned()).send()?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!(
                "failed to reqest: {} (HTTP: {})",
                self.endpoint,
                status
            ));
        }
        Ok(resp.text()?)
    }
}
