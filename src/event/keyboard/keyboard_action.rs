use glfw::Action;

#[derive(Debug)]
pub enum KeyboardAction {
    Press,
    Release,
    Repeat,
}

// TODO: This should probably be performed in a GLFW platform-specific module,
// so that GLFW would be easily swappable for another windowing library.
// Once this is done, the `use` statement above should be removed.
impl From<&Action> for KeyboardAction {
    fn from(glfw_action: &Action) -> Self {
        match glfw_action {
            Action::Press => KeyboardAction::Press,
            Action::Release => KeyboardAction::Release,
            Action::Repeat => KeyboardAction::Repeat,
        }
    }
}
