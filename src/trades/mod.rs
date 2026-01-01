use std::fmt::Display;

use miette::{IntoDiagnostic, Result};
use reqwest::{
    ClientBuilder,
    header::{AUTHORIZATION, HeaderMap},
};
use serde::Deserialize;

use crate::BASE_URL;
use crate::cli::Credentials;
pub mod browse;

#[derive(Debug)]
pub struct Token(String);

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Token {
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<AuthRes> for Token {
    fn from(value: AuthRes) -> Self {
        Self(value.access_token.to_string())
    }
}

pub async fn authenticate(creds: &Credentials) -> Result<Token> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        creds.authorization_header.parse().into_diagnostic()?,
    );
    let client = ClientBuilder::new().build().into_diagnostic()?;
    let mut url = reqwest::Url::parse(BASE_URL).into_diagnostic()?;
    url = url
        .join("/oauth/client_credential/accesstoken")
        .into_diagnostic()?;
    url.set_query(Some("grant_type=client_credentials"));
    let req = client.get(url).headers(headers);

    let res = req.send().await.into_diagnostic()?;
    let json = res.json::<AuthRes>().await.into_diagnostic()?;
    Ok(json.into())
}

#[derive(Deserialize, Debug)]
struct AuthRes {
    // refresh_token_expires_in: String,
    // api_product_list_json:String,
    // organization_name: String,
    // developer_email: String,
    // token_type: String,
    // issued_at: String,
    // client_id: String,
    access_token: String,
    // application_name: String,
    // scope: String,
    // expires_in: String,
    // refresh_count: String,
    // status: String
}
