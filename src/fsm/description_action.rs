use std::sync::{Arc, Mutex};

use crate::{
    fsm::{self, Window},
    App,
};

use super::TransitionAction;

pub async fn execute_desctiption_action(
    app: Arc<Mutex<App>>,
    action: TransitionAction,
) -> Option<fsm::Window> {
    // This does not involve any I/O. All actions happen immediately
    let mut app = app.lock().unwrap();
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
