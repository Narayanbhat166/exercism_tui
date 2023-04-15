use crate::{
    fsm::{self, TransitionInput, Window},
    App,
};
use crossterm::event::KeyCode;

use super::TransitionAction;

pub async fn execute_desctiption_action(
    app: &mut App,
    action: TransitionAction,
) -> Option<fsm::Window> {
    match action {
        TransitionAction::MoveDown => {
            // Check if text is overflowing, only then allow this
            if (app.description.current_height - app.description.scroll_offset.0 + 1)
                >= app.description.max_height
            {
                app.description.scroll_offset.0 += 1;
            }
            None
        }
        TransitionAction::MoveUp => {
            app.description.scroll_offset.0 = app.description.scroll_offset.0.saturating_sub(1);
            None
        }
        TransitionAction::Unselect => Some(Window::Exercises),
        TransitionAction::Nop | TransitionAction::Init | TransitionAction::Select => None,
    }
}
