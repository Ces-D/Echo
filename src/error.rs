pub enum EchoError {
    IoNamedTempFileError,
    ClientRequestError(String),
}
