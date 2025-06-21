use anyhow::Result;
use scraper::{ElementRef, Html, Selector};

pub struct HtmlParser {
    html: Html,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        let html = Html::parse_document(html);
        Self { html }
    }

    pub fn get_sample(&self) -> Result<Vec<(String, String)>> {
        let mut samples = vec![];
        let section_sel = Selector::parse("section").unwrap();
        for section in self.html.select(&section_sel) {
            let mut current_label = None;
            for node in section.children() {
                if let Some(elem) = ElementRef::wrap(node) {
                    match elem.value().name() {
                        "h3" => {
                            let txt = elem.text().collect::<String>().trim().to_owned();
                            current_label = Some(txt);
                        }
                        "pre" => {
                            let body = elem.text().collect::<String>().trim().to_owned();
                            if let Some(label) = current_label.take() {
                                if label.starts_with("入力例") {
                                    samples.push((body, String::new()));
                                } else if label.starts_with("出力例") {
                                    if let Some((_inp, out)) = samples.last_mut() {
                                        *out = body;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(samples)
    }
}
