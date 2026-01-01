use reqwest::{ClientBuilder, Url, header::HeaderMap};
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, cli::Credentials, trades::Token};
use miette::{IntoDiagnostic, Result, bail};

static BROWSE_ENDPOINT: &str = "/tradesregister/v1/browse";

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "camelCase"))]
pub struct TradeEntry {
    #[serde(rename(serialize = "Licence ID", deserialize = "licenceID"))]
    licence_id: String,
    licensee: String,
    licence_name: Option<String>,
    licence_number: Option<String>,
    licence_type: Option<String>,
    status: Status,
    suburb: Option<String>,
    postcode: Option<String>,
    business_names: Option<String>,
    categories: Option<String>,
    classes: Option<String>,
    expiry_date: Option<String>,
    // historical_licence_numbers: Option<Vec<String>>,
    // abn: Option<String>,
    // acn: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct BuilderBasic {
    licence_id: String,
    licence_number: String,
    licence_type: String,
    licence_type_friendly: String,
    licence_group: String,
    status: String,
    granted: String,
    expires: String,
    licensee: String,
    licensee_type: String,
    suburb: String,
    state: String,
    postcode: String,
    address: String,
    abn: Option<String>,
    acn: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    Current,
    Expired,
    Surrendered,
    Suspended,
    Refused,
    Lapsed,
    Cancelled,
    Deregulated,
}

pub async fn browse(
    creds: &Credentials,
    token: &Token,
    search_text: &str,
) -> Result<Vec<TradeEntry>> {
    let mut url = Url::parse(BASE_URL)
        .into_diagnostic()?
        .join(BROWSE_ENDPOINT)
        .into_diagnostic()?;
    url.set_query(Some(&format!("searchText={search_text}")));
    let client = ClientBuilder::new().build().into_diagnostic()?;
    let mut headers = HeaderMap::new();
    headers.insert("apikey", creds.api_key.as_str().parse().into_diagnostic()?);
    let mut req = client.get(url);
    req = req.bearer_auth(token);
    req = req.headers(headers);
    if search_text.chars().count() < 2 {
        miette::bail!(
            help = "Search text needs to have at least 2 characters.",
            "Below minimum character count: {}",
            search_text
        );
    }
    let res = req.send().await.into_diagnostic()?;
    if res.status().is_success() {
        // println!("{:#?}", res.text().await.into_diagnostic()?);
        res.json::<Vec<TradeEntry>>().await.into_diagnostic()
    } else {
        bail!(res.text().await.into_diagnostic()?)
    }
}
