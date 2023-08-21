use crate::{count::Count, crawl, rating::Rating};
use anyhow::{anyhow, bail};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufRead, Write},
};
use url::Url;

pub fn read(path: &Utf8Path) -> anyhow::Result<Fics> {
    let file = open(path)?;
    let fics = serde_yaml::from_reader(&file)?;

    Ok(Fics { file, fics })
}

fn open(path: &Utf8Path) -> io::Result<File> {
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}

#[derive(Debug)]
pub struct Fics {
    file: File,
    fics: Vec<Fic>,
}

impl Fics {
    pub fn count(&self) -> usize {
        self.fics.len()
    }

    pub async fn crawl(&mut self) -> anyhow::Result<()> {
        for fic in &mut self.fics {
            fic.crawl().await?;
        }

        Ok(())
    }

    pub async fn add(&mut self, mut fic: Fic, crawl: bool) -> anyhow::Result<()> {
        if crawl {
            fic.crawl().await?;
            println!("Adding \"{}\"...", fic.name()?);
        } else {
            println!("Adding \"{}\"... (without crawling)", fic.url);
        }

        self.check_unique(&fic)?;

        serde_yaml::to_writer(&self.file, &[&fic])?;
        self.file.flush()?;

        self.fics.push(fic);

        Ok(())
    }

    fn check_unique(&self, fic: &Fic) -> anyhow::Result<()> {
        if self.fics.iter().any(|f| f.url == fic.url) {
            bail!("Fic \"{}\" is already in list.", fic.name_or_url());
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fic {
    url: Url,
    rating: Rating,
    about: String,

    // Fields loaded from crawling.
    #[serde(skip_serializing)]
    name: Option<String>,
    #[serde(skip_serializing)]
    words: Option<Count>,
}

impl Fic {
    pub fn new(url: Url, rating: Rating) -> anyhow::Result<Self> {
        Ok(Self {
            url,
            rating,
            about: read_about()?,
            name: None,
            words: None,
        })
    }

    pub fn name(&self) -> anyhow::Result<&str> {
        self.crawled("name", self.name.as_deref())
    }

    pub fn name_or_url(&self) -> &str {
        self.name.as_deref().unwrap_or(self.url.as_str())
    }

    pub async fn crawl(&mut self) -> anyhow::Result<()> {
        let data = crawl::crawl(&self.url).await?;

        self.name = Some(data.name);
        self.words = Some(data.words);

        Ok(())
    }

    fn crawled<'a, T>(&'a self, field: &str, value: Option<&'a T>) -> anyhow::Result<&'a T>
    where
        T: ?Sized,
    {
        value.ok_or_else(|| anyhow!("Tried to access crawled field `{field}` before load."))
    }
}

fn read_about() -> anyhow::Result<String> {
    println!("Enter the \"about\" for this fic. Press ENTER when done.");

    let mut about = String::new();
    io::stdin().lock().read_line(&mut about)?;

    Ok(about)
}
