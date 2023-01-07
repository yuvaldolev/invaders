use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Glfw, Key, WindowEvent, WindowMode};

use crate::error;

pub struct Window {
    title: String,
    width: u32,
    height: u32,
    glfw_token: Glfw,
    glfw_window: glfw::Window,
    glfw_event_receiver: Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> error::Result<Self> {
        // Initialize GLFW.
        // This game only supports having a single window, and thus
        // the GLFW library is initialized when the window is created.
        let mut glfw_token =
            glfw::init(glfw::FAIL_ON_ERRORS).map_err(error::Error::GlfwInitialize)?;

        // Create the GLFW window.
        let (mut glfw_window, glfw_event_receiver) = glfw_token
            .create_window(width, height, &title, WindowMode::Windowed)
            .ok_or(error::Error::GlfwCreateWindow)?;

        // Make the window's context the current context.
        glfw_window.make_current();

        // Capture key events.
        glfw_window.set_key_polling(true);

        Ok(Self {
            title,
            width,
            height,
            glfw_token,
            glfw_window,
            glfw_event_receiver,
        })
    }

    pub fn update(&mut self) {
        // Poll for and process events
        self.glfw_token.poll_events();
        for (_, event) in glfw::flush_messages(&self.glfw_event_receiver) {
            println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.glfw_window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }
}
