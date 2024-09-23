use std::io::{self};

use log::error;
use tempfile::NamedTempFile;

const TEMP_NAME_PREFIX: &str = "echo-";
pub fn create_app_temp_file(prefix: &str) -> io::Result<NamedTempFile> {
    let app_named_prefix = format!("{}{}", TEMP_NAME_PREFIX, prefix);
    match tempfile::Builder::new()
        .prefix(app_named_prefix.as_str())
        .tempfile()
    {
        Ok(file) => Ok(file),
        Err(err) => {
            error!("Failure to create temp file: {}", app_named_prefix);
            Err(err)
        }
    }
}
