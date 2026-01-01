mod cli;
pub use cli::{Api, Cli, print_completions};
mod output;
pub use output::write_csv;
pub mod trades;

pub static BASE_URL: &str = "https://api.onegov.nsw.gov.au";
