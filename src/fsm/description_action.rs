use crate::{
    api::{self},
    fsm::{self, TransitionInput, Window},
    App,
};
use crossterm::event::KeyCode;

pub enum DescriptionAction {
    ScrollDown,
    ScrollUp,
    GoBack,
    Nop,
}

impl DescriptionAction {
    pub async fn execute_action(&self, app: &mut App) -> Option<fsm::Window> {
        match self {
            DescriptionAction::ScrollDown => {
                // Check if text is overflowing, only then allow this
                if (app.description.current_height - app.description.scroll_offset.0 + 1)
                    >= app.description.max_height
                {
                    app.description.scroll_offset.0 += 1;
                }
                None
            }
            DescriptionAction::ScrollUp => {
                app.description.scroll_offset.0 = app.description.scroll_offset.0.saturating_sub(1);
                None
            }
            DescriptionAction::GoBack => Some(Window::Exercises),
            DescriptionAction::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> Self {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => Self::ScrollDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => Self::ScrollUp,
            TransitionInput::Key(KeyCode::Left | KeyCode::Char('h')) => Self::GoBack,
            _ => Self::Nop,
        }
    }
}
