use google_sheets4::{
    hyper::client::HttpConnector,
    hyper_rustls::HttpsConnector,
    oauth2::{
        authenticator::Authenticator, read_application_secret, InstalledFlowAuthenticator,
        InstalledFlowReturnMethod,
    },
};

type Auth = Authenticator<HttpsConnector<HttpConnector>>;

const SECRETS_JSON: &str = "fics.secrets.json";
const TOKENS_CACHE: &str = "fics.tokens.cache";

pub async fn login(port: u16) -> anyhow::Result<Auth> {
    let secret = read_application_secret(SECRETS_JSON).await?;
    let port = InstalledFlowReturnMethod::HTTPPortRedirect(port);
    let auth = InstalledFlowAuthenticator::builder(secret, port)
        .persist_tokens_to_disk(TOKENS_CACHE)
        .build()
        .await?;

    Ok(auth)
}
