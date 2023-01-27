use std::result;

use glfw::InitError;
use wgpu::{CreateSurfaceError, RequestDeviceError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed initializing GLFW: {0}")]
    GlfwInitialize(#[source] InitError),

    #[error("failed creating GLFW window")]
    GlfwCreateWindow,

    #[error("failed creating wgpu surface: {0}")]
    WgpuCreateSurface(#[source] CreateSurfaceError),

    #[error("failed requesting wgpu adapter")]
    WgpuRequestAdapter,

    #[error("failed requesting wgpu device: {0}")]
    WgpuRequestDevice(#[source] RequestDeviceError),
}

pub type Result<T> = result::Result<T, Error>;
