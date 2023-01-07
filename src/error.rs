use std::result;

use glfw::InitError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed initializing GLFW: {0}")]
    GlfwInitialize(#[source] InitError),

    #[error("failed creating GLFW window")]
    GlfwCreateWindow,
}

pub type Result<T> = result::Result<T, Error>;
