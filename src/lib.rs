mod cli;
pub use cli::{Api, Cli, print_completions};
pub mod trades;

pub static BASE_URL: &str = "https://api.onegov.nsw.gov.au";
