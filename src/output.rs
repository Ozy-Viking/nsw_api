use std::{io, path::PathBuf};

use csv::Writer;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

pub fn write_csv<T: Serialize>(data: Vec<T>, path: Option<PathBuf>) -> Result<()> {
    if let Some(p) = path {
        let mut wtr = Writer::from_path(p).into_diagnostic()?;
        for row in data {
            wtr.serialize(row).into_diagnostic()?;
        }
        wtr.flush().into_diagnostic()?;
    } else {
        let mut wtr = Writer::from_writer(io::stdout());
        for row in data {
            wtr.serialize(row).into_diagnostic()?;
        }
        wtr.flush().into_diagnostic()?;
    }
    Ok(())
}
