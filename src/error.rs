use std::result;

use winit::error::OsError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed creating a window: {0}")]
    CreateWindow(#[source] OsError),
}

pub type Result<T> = result::Result<T, Error>;
