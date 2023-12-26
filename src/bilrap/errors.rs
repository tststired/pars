// Box<dyn Error> sucks, wasteful overhead monomorphic better
#[derive(Clone)]
pub enum MErrors {
    ParseError(String),
    UsageError,
    UTF8Error,
    JoinError(String),
}

impl std::error::Error for MErrors {}

impl std::fmt::Display for MErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MErrors::ParseError(parse_error) => write!(f, "ParseError: {}", parse_error),
            MErrors::UsageError => write!(
                f,
                "UsageError: bilrap (-r [address]:[port]/[jobs]) | (-j [jobs])"
            ),
            MErrors::UTF8Error => write!(f, "UTF8Error: Invalid UTF-8 string"),
            MErrors::JoinError(str) => write!(f, "JoinError: {}", str),
        }
    }
}

impl std::fmt::Debug for MErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl From<std::io::Error> for MErrors {
    fn from(_error: std::io::Error) -> Self {
        MErrors::UTF8Error
    }
}

impl From<std::num::ParseIntError> for MErrors {
    fn from(_error: std::num::ParseIntError) -> Self {
        MErrors::UTF8Error
    }
}

impl From<tokio::sync::mpsc::error::SendError<String>> for MErrors {
    fn from(_error: tokio::sync::mpsc::error::SendError<String>) -> Self {
        MErrors::UTF8Error
    }
}

impl From<tokio::task::JoinError> for MErrors {
    fn from(_error: tokio::task::JoinError) -> Self {
        MErrors::JoinError(_error.to_string())
    }
}
