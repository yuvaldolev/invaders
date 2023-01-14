use glfw::Key;

#[derive(Debug)]
pub enum KeyboardKey {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ShiftLeft,
    Unknown,
}

// TODO: This should probably be performed in a GLFW platform-specific module,
// so that GLFW would be easily swappable for another windowing library.
// Once this is done, the `use` statement above should be removed.
impl From<&Key> for KeyboardKey {
    fn from(glfw_key: &Key) -> Self {
        match glfw_key {
            Key::Down => KeyboardKey::ArrowDown,
            Key::Left => KeyboardKey::ArrowLeft,
            Key::Right => KeyboardKey::ArrowRight,
            Key::Up => KeyboardKey::ArrowUp,
            Key::LeftShift => KeyboardKey::ShiftLeft,
            _ => KeyboardKey::Unknown,
        }
    }
}
