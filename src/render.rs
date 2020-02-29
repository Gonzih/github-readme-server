use comrak::{markdown_to_html, ComrakOptions};
use reqwest;

fn raw_readme_url(org: &str, repo: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/master/README.md",
        org, repo
    )
}

pub async fn load_readme(org: &str, repo: &str) -> String {
    let txt = reqwest::get(&raw_readme_url(org, repo))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    txt
}

pub struct Render {
    org: String,
    repo: String,
    txt: String,
}

impl Render {
    pub async fn new(org: String, repo: String) -> Self {
        let txt = load_readme(&org, &repo).await;
        Self { org, repo, txt }
    }

    pub fn render_markdown(&self) -> String {
        markdown_to_html(&self.txt, &ComrakOptions::default())
    }

    pub fn render(&self) -> String {
        self.render_markdown()
    }

    pub fn title(&self) -> String {
        for line in self.txt.lines() {
            if line.starts_with('#') {
                return line.trim_matches('#').trim().to_string()
            }
        }

        format!("{}/{}", self.org, self.repo)
    }
}
