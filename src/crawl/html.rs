use std::str::FromStr;

use anyhow::{anyhow, bail};

pub struct Html(scraper::Html);

impl Html {
    pub fn new(document: &str) -> Self {
        Self(scraper::Html::parse_document(document))
    }

    pub fn get(&self, selector: &str) -> anyhow::Result<Elem<'_>> {
        get!(selector, self.0)
    }
}

pub struct Elem<'a>(scraper::ElementRef<'a>);

impl<'a> Elem<'a> {
    pub fn get(&self, selector: &str) -> anyhow::Result<Elem<'_>> {
        get!(selector, self.0)
    }

    pub fn text(&self) -> String {
        self.0.text().collect::<String>().trim().into()
    }

    pub fn as_usize(&self) -> Result<usize, <usize as FromStr>::Err> {
        self.text().replace(',', "").parse()
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.text().parse()
    }
}

// === impls ===

macro_rules! get {
    ($s:expr, $node:expr) => {{
        let s = $s;
        let selector = make_selector(s)?;
        let mut iter = $node.select(&selector);

        let item = iter
            .next()
            .ok_or_else(|| anyhow!("Expected element: `{s}`"))?;

        if iter.next().is_some() {
            bail!("Expected element `{s}` to be unique on page.");
        }

        Ok(Elem(item))
    }};
}

use get;

fn make_selector(s: &str) -> anyhow::Result<scraper::Selector> {
    scraper::Selector::parse(s).map_err(|e| anyhow!("{e}"))
}
