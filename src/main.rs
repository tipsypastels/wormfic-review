use self::{
    config::Config,
    fics::{Fic, Fics},
    rating::Rating,
    sheets::Sheet,
};
use camino::Utf8PathBuf;
use structopt::StructOpt;
use url::Url;

mod config;
mod count;
mod crawl;
mod fics;
mod rating;
mod sheets;

#[derive(StructOpt, Debug)]
#[structopt(about = "dakota's tool for building her wormfic spreadsheet")]
struct WormficReview {
    #[structopt(default_value = "fics.config.toml")]
    config_file: Utf8PathBuf,

    #[structopt(default_value = "fics.yml")]
    fics_file: Utf8PathBuf,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add(AddCommand),
    Crawl(CrawlCommand),
    Upload,
    Check,
}

#[derive(StructOpt, Debug)]
struct AddCommand {
    url: Url,
    rating: Rating,

    #[structopt(long = "no-upload")]
    no_upload: bool,

    #[structopt(long = "no-crawl")]
    no_crawl: bool,
}

#[derive(StructOpt, Debug)]
struct CrawlCommand {
    url: Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = WormficReview::from_args();
    let config = config::read(&opt.config_file)?;
    let mut fics = fics::read(&opt.fics_file)?;

    match opt.command {
        Command::Add(cmd) => add(&config, &mut fics, cmd).await,
        Command::Crawl(cmd) => crawl(cmd).await,
        Command::Upload => upload(&config, &fics).await,
        Command::Check => check(&mut fics).await,
    }
}

async fn add(config: &Config, fics: &mut Fics, cmd: AddCommand) -> anyhow::Result<()> {
    let fic = Fic::new(cmd.url, cmd.rating)?;

    fics.add(fic, !cmd.no_crawl).await?;

    if !cmd.no_upload {
        upload(config, fics).await?;
    }

    Ok(())
}

async fn crawl(cmd: CrawlCommand) -> anyhow::Result<()> {
    let data = crawl::crawl(&cmd.url).await?;

    println!("{data:#?}");
    Ok(())
}

async fn upload(config: &Config, fics: &Fics) -> anyhow::Result<()> {
    let sheet = Sheet::new(config).await?;

    Ok(())
}

async fn check(fics: &mut Fics) -> anyhow::Result<()> {
    fics.crawl().await?;

    println!("Ok! {} fic(s) in CSV.", fics.count());
    Ok(())
}
