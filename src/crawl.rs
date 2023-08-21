use self::html::Html;
use crate::count::Count;
use anyhow::anyhow;
use url::Url;

mod ao3;
mod html;

// Should be the same as the API-loaded fields on `Fic`, which this gets flattened into.
// Except not wrapped in `Option` unless it missing is considered valid.
#[derive(Debug)]
pub struct Crawl {
    pub name: String,
    pub words: Count,
}

pub async fn crawl(url: &Url) -> anyhow::Result<Crawl> {
    let response = reqwest::get(url.as_str()).await?;
    let response = response.error_for_status()?;
    let body = response.text().await?;
    let html = Html::new(&body);

    match url.domain() {
        Some("archiveofourown.org") => ao3::crawl(&html),

        Some(d) => Err(anyhow!("don't know how to crawl domain: `{d}`")),
        None => Err(anyhow!("can't crawl URL without domain!")),
    }
}
