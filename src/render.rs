use comrak::{markdown_to_html, ComrakOptions};
use reqwest;

pub struct Render {
    org: String,
    repo: String,
}

impl Render {
    pub fn new(org: String, repo: String) -> Self {
        Self { org, repo }
    }

    fn raw_readme_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/master/README.md",
            self.org, self.repo
        )
    }

    pub async fn load_readme(&self) -> String {
        let txt = reqwest::get(&self.raw_readme_url())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        txt
    }

    pub async fn render_markdown(&self) -> String {
        markdown_to_html(&self.load_readme().await, &ComrakOptions::default())
    }

    pub async fn render(&self) -> String {
        self.render_markdown().await
    }
}
