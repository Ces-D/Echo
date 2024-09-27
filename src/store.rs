use std::fs::File;
use std::io::{self};
use std::path::PathBuf;

use dirs::document_dir;
use log::error;
use tempfile::NamedTempFile;

use echo::error::EchoError;

const NAME_PREFIX: &str = "echo_";
const ECHO_DOCUMENT_STORE: &str = "store";

pub fn create_app_temp_file(prefix: &str) -> io::Result<NamedTempFile> {
    let app_named_prefix = format!("{}{}", NAME_PREFIX, prefix);
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

pub fn create_stored_file_path(file_name: &String) -> Option<PathBuf> {
    let document_dir_path = document_dir()?;
    let store_path = document_dir_path.join(format!("{}{}", NAME_PREFIX, ECHO_DOCUMENT_STORE));
    if !store_path.is_dir() {
        std::fs::create_dir(store_path.clone()).ok()?;
    }
    let file_path = store_path.join(format!("{}{}", NAME_PREFIX, file_name));
    Some(file_path)
}

/// Open a file as either write, append, or readonly. The file is ensured to at least exist in the
/// echo_store
pub fn open_stored_file(
    file_name: &String,
    readonly: bool,
    overwrite: bool,
) -> Result<File, EchoError> {
    match create_stored_file_path(&file_name) {
        Some(file_path) => {
            if readonly {
                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .read(true)
                    .open(file_path)
                    .map_err(|error| EchoError::IoStoredFileError(error.to_string()))?;
                Ok(file)
            } else {
                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .write(overwrite)
                    .append(!overwrite)
                    .open(file_path)
                    .map_err(|error| EchoError::IoStoredFileError(error.to_string()))?;
                Ok(file)
            }
        }
        None => Err(EchoError::IoStoredFileError(format!(
            "Unable to create the stored file path: {}",
            file_name
        ))),
    }
}
