use crate::error;
use crate::window::Window;

pub struct Invaders {
    window: Window,
}

impl Invaders {
    pub fn new() -> error::Result<Self> {
        Ok(Self {
            window: Window::new(1280, 720)?,
        })
    }
}
