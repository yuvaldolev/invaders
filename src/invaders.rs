use glam::Vec2;

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

            // Process the input.
            // Update the window events.
            self.window.update(&mut frame_events);

            // Simulate the game.
            // TODO: This should be moved to dedicated Game code:
            // ```
            // game.simulate(&frame_events);
            // ```
            for event in frame_events.iter() {
                log::debug!("Event: {:?}", event);
            }

            // Render the game.
            // TODO: This should be moved to dedicated Game code
            // ```
            // game.render(&self.renderer);
            // ```
            self.renderer.draw_quad(Vec2::new();

            // Swap the window buffers.
            self.window.swap_buffers();
        }
    }
}
