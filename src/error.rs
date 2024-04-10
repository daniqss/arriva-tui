#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn from(message: String) -> Self {
        Self {
            message
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(error: serde_urlencoded::de::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}