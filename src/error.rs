use thiserror::Error;
use tokio::io;

/// Custom Result type that uses `crate::error::Error` as the error type
pub type Result<T> = std::result::Result<T, Error>;

/// A custom error type that deals with invalid operations in TexCreate
#[derive(Error, Debug)]
pub enum Error {
    // This will occur if the user puts in a template that doesn't exist in a repo
    #[error("The Template `{0}` is Invalid!")]
    InvalidTemplate(String),
    // This will occur if the user inputs text in a prompt that is invalid
    #[error("The input `{0}` is Invalid!")]
    InvalidInput(String),
    // This will occur if the user puts in a repo that isn't `mkproj` or `custom`
    #[error("The repo `{0}` is invalid, only `mkproj` or `custom` is allowed!")]
    InvalidRepo(String),
    // This will handle any IO Error
    #[error("IO Error")]
    IO(#[from] io::Error),
}
