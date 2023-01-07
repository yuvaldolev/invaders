use crate::error;
use crate::window::Window;

pub struct Invaders {
    // event_loop: EventLoop<()>,
    window: Window,
}

impl Invaders {
    pub fn new() -> error::Result<Self> {
        // Create the window.
        let window = Window::new(String::from("Invaders"), 1280, 720)?;

        // Initialize the game.
        Ok(Self { window })
    }

    pub fn run(&mut self) {
        loop {
            self.window.update();
            self.window.swap_buffers();
        }
    }
}
