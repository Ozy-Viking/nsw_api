use std::io;

use clap::{Args, Command, Parser, Subcommand};
use clap_complete::aot::generate;
use clap_complete::{Generator, Shell};

#[derive(Debug, Subcommand)]
pub enum Api {
    /// Trades licensing information
    Trades {
        #[command(flatten)]
        creds: Credentials,
    },
    AutoComplete {
        shell: Shell,
    },
}

#[derive(Debug, Args)]
pub struct Credentials {
    // Api Key from [api.nsw](https://api.nsw.gov.au/DeveloperApp)
    #[arg(env, short, long)]
    pub api_key: String,
    /// Api Secret from [api.nsw](https://api.nsw.gov.au/DeveloperApp)
    #[arg(env, short = 's', long)]
    pub api_secret: String,
    /// Authorization Header from [api.nsw](https://api.nsw.gov.au/DeveloperApp)
    #[arg(env, long = "auth")]
    pub authorization_header: String,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub api: Api,
}

pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}
