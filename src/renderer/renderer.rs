use crate::error;
use crate::renderer::render_command::RenderCommand;
use crate::window::Window;

pub struct Renderer {
    render_command: RenderCommand,
}

impl Renderer {
    pub fn new(window: &Window) -> error::Result<Self> {
        // Create the render command.
        let render_command = RenderCommand::new(window)?;

        // Initialize Renderer.
        Ok(Self { render_command })
    }
}
