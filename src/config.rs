use anyhow::Context;
use camino::Utf8Path;
use serde::Deserialize;
use std::fs;

pub fn read(path: &Utf8Path) -> anyhow::Result<Config> {
    let ctx = |verb| move || format!("Failed to {verb} config file: {path}.");
    let text = fs::read_to_string(path).with_context(ctx("read"))?;
    let config = toml::from_str(&text).with_context(ctx("parse"))?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Config {}
