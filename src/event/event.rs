use crate::event::keyboard::KeyboardEvent;

#[derive(Debug)]
pub enum Event {
    Keyboard(KeyboardEvent),
}
