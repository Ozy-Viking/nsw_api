use std::io::{self, Write};

use clap::{CommandFactory as _, Parser};
use clap_complete::generate;
use miette::{IntoDiagnostic, Result};
use nsw::{
    Api, Cli,
    trades::{authenticate, browse::browse},
    write_csv,
};

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_panic_hook();
    let _ = dotenvy::dotenv();
    let mut command = Cli::command();
    let name = Cli::command().get_name().to_string();

    let cli = Cli::parse();
    cli.color.write_global();
    match cli.api {
        Api::AutoComplete { shell } => {
            let mut buff = io::stdout();
            generate(shell, &mut command, name, &mut buff);
            buff.flush().into_diagnostic()?;
            return Ok(());
        }
        Api::Trades {
            creds,
            search_terms,
            csv_output,
        } => {
            let token = authenticate(&creds).await?;
            let results = browse(&creds, &token, search_terms.unwrap_or_default().as_str()).await?;
            write_csv(results, csv_output)?;
        }
    }
    Ok(())
}
