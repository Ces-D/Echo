use std::fs::File;
use std::path::PathBuf;

use dirs::document_dir;

use echo::error::EchoError;

const NAME_PREFIX: &str = "echo_";
const ECHO_DOCUMENT_STORE: &str = "store";

fn create_store_file_name(identifier: &String) -> String {
    format!("{}{}.toml", NAME_PREFIX, identifier)
}

/// Ensures the apps store dir exists but does not ensure the file exists.
/// Call `open_stored_file` to ensure all paths exist
pub fn stored_file_path(identifier: &String) -> Option<PathBuf> {
    let document_dir_path = document_dir()?;
    let store_path = document_dir_path.join(format!("{}{}", NAME_PREFIX, ECHO_DOCUMENT_STORE));
    if !store_path.is_dir() {
        std::fs::create_dir(store_path.clone()).ok()?;
    }
    let file_path = store_path.join(create_store_file_name(identifier));
    Some(file_path)
}

/// Open a file as either write, append, or readonly. The file is ensured to at least exist in the
/// echo_store
pub fn open_stored_file(
    identifier: &String,
    readonly: bool,
    overwrite: bool,
) -> Result<File, EchoError> {
    match stored_file_path(identifier) {
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
            "Unable to create the stored file path",
        ))),
    }
}
