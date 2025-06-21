use anyhow::{Result, anyhow};

pub const ATCODER_BASE_URL: &str = "https://atcoder.jp/contests";

pub struct AtcoderClient {
    pub endpoint: String,
    pub client: reqwest::blocking::Client,
}

impl AtcoderClient {
    pub fn new(contest_name: &str, problem: &str) -> Self {
        let endpoint = format!("{ATCODER_BASE_URL}/{contest_name}/tasks/{contest_name}_{problem}");
        Self {
            endpoint,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_html(&self) -> Result<String> {
        let resp = self.client.get(&self.endpoint).send()?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!(
                "リクエスト失敗: {} (HTTP: {})",
                self.endpoint,
                status
            ));
        }
        Ok(resp.text()?)
    }
}
