use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("set data duplicate")]
    Duplicate,

    #[error("other error")]
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Storage {
    fn set_if_absent(&self, long_url: &str, short_url: &str) -> Result<Option<()>>;
    fn get_content(&self, short_url: &str) -> Result<Option<String>>;
}
