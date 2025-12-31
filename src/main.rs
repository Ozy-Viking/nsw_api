use std::io;

use clap::{CommandFactory as _, Parser};
use clap_complete::generate;
use miette::Result;
use nsw::{Api, Cli, trades::authenticate};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_panic_hook();
    let _ = dotenvy::dotenv();
    let mut command = Cli::command();

    let cli = Cli::parse();
    match cli.api {
        Api::AutoComplete { shell } => {
            generate(shell, &mut command, "nsw", &mut io::stdout());
            return Ok(());
        }
        Api::Trades { creds } => {
            let res = authenticate(&creds).await;
            println!("{:?}", res);
        }
    }
    Ok(())
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

#[derive(Debug, PartialEq, Eq)]
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
