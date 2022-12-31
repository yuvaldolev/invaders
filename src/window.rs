use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{self, WindowBuilder};

use crate::error;

pub struct Window {
    event_loop: EventLoop<()>,
    handle: window::Window,
}

impl Window {
    pub fn new(width: u32, height: u32) -> error::Result<Self> {
        let event_loop = EventLoop::new();
        let handle = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop)
            .map_err(error::Error::CreateWindow)?;

        Ok(Self { event_loop, handle })
    }
}
