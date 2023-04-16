use std::{
    env::current_exe,
    sync::{Arc, Mutex},
    thread::current,
};

use crate::{
    api::{self},
    fsm::{self, Window},
    App,
};

use super::TransitionAction;

pub async fn execute_exercise_action(
    app: Arc<Mutex<App>>,
    action: TransitionAction,
) -> Option<fsm::Window> {
    match action {
        TransitionAction::MoveDown => {
            let mut app = app.lock().unwrap();
            app.exercises.next();
            None
        }
        TransitionAction::MoveUp => {
            let mut app = app.lock().unwrap();
            app.exercises.previous();
            None
        }
        TransitionAction::Select => {
            let (current_track, current_exercise) = {
                let mut app = app.lock().unwrap();
                let current_exercise = app.exercises.get_current_item().unwrap();

                app.bottom_bar = Some(format!(
                    "Fetching description for {} exercise",
                    current_exercise.title
                ));
                (app.tracks.get_current_item().unwrap(), current_exercise)
            };

            // Involves I/O
            let description_text = api::description::get_description::get_description(
                current_track.slug,
                current_exercise.slug,
            )
            .await
            .unwrap();

            {
                let mut app = app.lock().unwrap();
                app.description.text = description_text;
                app.bottom_bar = None;
            }

            // Change current active window to Description
            Some(Window::Description)
        }
        TransitionAction::Unselect => Some(Window::Tracks),
        TransitionAction::Nop | TransitionAction::Init => None,
    }
}
