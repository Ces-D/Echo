use std::fmt::Display;

pub enum EchoError {
    ClientRequestError(String),
    CliParamError(String),
}

impl Display for EchoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EchoError::ClientRequestError(e) => write!(f, "ClientRequestError: {}", e),
            EchoError::CliParamError(e) => write!(f, "CliParamError: {}", e),
        }
    }
}
