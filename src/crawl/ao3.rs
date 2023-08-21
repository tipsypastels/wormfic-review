use super::{Crawl, Html};

pub fn crawl(html: &Html) -> anyhow::Result<Crawl> {
    let name = html.get(".title.heading")?.text();
    let stats = html.get("dl.stats")?;
    let words = stats.get("dd.words")?.parse()?;

    Ok(Crawl { name, words })
}
