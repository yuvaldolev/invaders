use crate::error;
use crate::renderer::Renderer;
use crate::window::Window;

pub struct Invaders {
    window: Window,
    renderer: Renderer,
}

impl Invaders {
    pub fn new() -> error::Result<Self> {
        // Create the window.
        let window = Window::new(String::from("Invaders"), 1280, 720)?;

        // Create the renderer.
        let renderer = Renderer::new(&window)?;

        // Initialize Invaders.
        Ok(Self { window, renderer })
    }

    pub fn run(&mut self) {
        let mut frame_events = Vec::new();

        loop {
            // Clear the frame events.
            frame_events.clear();

            // Update the window events.
            self.window.update(&mut frame_events);

            // Simulate the game.
            for event in frame_events.iter() {
                log::debug!("Event: {:?}", event);
            }

            // Swap the window buffers.
            self.window.swap_buffers();
        }
    }
}
