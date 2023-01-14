use crate::event::keyboard::keyboard_action::KeyboardAction;
use crate::event::keyboard::keyboard_key::KeyboardKey;

#[derive(Debug)]
pub struct KeyboardEvent {
    key: KeyboardKey,
    action: KeyboardAction,
}

impl KeyboardEvent {
    pub fn new(key: KeyboardKey, action: KeyboardAction) -> Self {
        Self { key, action }
    }
}
