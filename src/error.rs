#[derive(thiserror::Error, Debug)]

pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
