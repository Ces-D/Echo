use std::fmt::Display;

use rspotify::{model::IdError, ClientError};

pub enum EchoError {
    ClientRequestError(String),
    CliParamError(String),
    DatabaseError(String),
}

impl Display for EchoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EchoError::ClientRequestError(e) => write!(f, "ClientRequestError: {}", e),
            EchoError::CliParamError(e) => write!(f, "CliParamError: {}", e),
            EchoError::DatabaseError(e) => write!(f, "DatabaseError: {}", e),
        }
    }
}

impl From<ClientError> for EchoError {
    fn from(value: ClientError) -> Self {
        EchoError::ClientRequestError(value.to_string())
    }
}

impl From<IdError> for EchoError {
    fn from(value: IdError) -> Self {
        match value {
            IdError::InvalidId => EchoError::CliParamError(IdError::InvalidId.to_string()),
            IdError::InvalidType => EchoError::CliParamError(IdError::InvalidType.to_string()),
            IdError::InvalidFormat => EchoError::CliParamError(IdError::InvalidFormat.to_string()),
            IdError::InvalidPrefix => EchoError::CliParamError(IdError::InvalidPrefix.to_string()),
        }
    }
}
