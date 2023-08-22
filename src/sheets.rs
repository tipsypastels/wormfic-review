use crate::config::Config;
use google_sheets4::{hyper, hyper_rustls, Sheets};

mod auth;

pub struct Sheet {}

impl Sheet {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let auth = auth::login(config.port).await?;
        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        );

        let s = hub.spreadsheets().get(&config.spreadsheet).doit().await?;

        dbg!(s);

        todo!()
    }
}
