use std::fmt::Display;

pub enum EchoError {
    IoNamedTempFileError(String),
    IoStoredFileError(String),
    ClientRequestError(String),
}

impl Display for EchoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EchoError::IoNamedTempFileError(e) => write!(f, "IoNamedTempFileError: {}", e),
            EchoError::ClientRequestError(e) => write!(f, "ClientRequestError: {}", e),
            EchoError::IoStoredFileError(e) => write!(f, "IoStoredFileError: {}", e),
        }
    }
}
