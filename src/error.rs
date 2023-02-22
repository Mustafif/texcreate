use thiserror::Error;
use tokio::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The Template `{0}` is Invalid!")]
    InvalidTemplate(String),
    #[error("The input `{0}` is Invalid!")]
    InvalidInput(String),
    #[error("IO Error")]
    IOError(#[from] io::Error),
}
