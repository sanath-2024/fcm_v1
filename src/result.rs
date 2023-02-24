pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Auth,
    Config,
    Deserialization,
    FCM(String),
    Timeout,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Auth => write!(f, "authentication error"),
            Error::Config => write!(f, "configuration error"),
            Error::Deserialization => write!(f, "deserialization error"),
            Error::FCM(msg) => write!(f, "firebase error: {}", msg),
            Error::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::error::Error for Error {}
