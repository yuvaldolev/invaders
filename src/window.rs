use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Glfw, Key, WindowEvent, WindowMode};

use crate::error;
use crate::event::{Event, KeyboardEvent};

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

    pub fn update(&mut self, frame_events: &mut Vec<Event>) {
        // Poll for and process events
        self.glfw_token.poll_events();
        for (_, event) in glfw::flush_messages(&self.glfw_event_receiver) {
            Self::process_event(&event, frame_events);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }

    fn process_event(event: &WindowEvent, frame_events: &mut Vec<Event>) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                Self::process_key_event(key, action, frame_events)
            }
            _ => (),
        }
    }

    fn process_key_event(key: &Key, action: &Action, frame_events: &mut Vec<Event>) {
        frame_events.push(Event::Keyboard(KeyboardEvent::new(
            key.into(),
            action.into(),
        )));
    }
}
