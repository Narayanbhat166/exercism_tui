use std::sync::{Arc, Mutex};

use crate::{api, fsm, App};

use super::{TransitionAction, Window};

pub async fn execute_track_action(
    app: Arc<Mutex<App>>,
    action: TransitionAction,
) -> Option<Window> {
    match action {
        TransitionAction::MoveDown => {
            let mut app = app.lock().unwrap();
            app.tracks.next();
            None
        }
        TransitionAction::MoveUp => {
            let mut app = app.lock().unwrap();
            app.tracks.previous();
            None
        }
        TransitionAction::Init => {
            // Involves I/O
            let all_tracks = api::tracks::get_tracks::get_tracks().await.unwrap();

            let mut app = app.lock().unwrap();
            app.tracks.add_items(all_tracks.tracks);
            app.tracks.state.select(Some(0));

            None
        }
        TransitionAction::Select => {
            let current_track = {
                let app = app.lock().unwrap();
                app.tracks.get_current_item().unwrap()
            };

            // This is an async task and involves I/O.
            // The lock should not be acquired before this
            // as it will be blocked untill the I/O finishes.
            let exercises = api::exercises::get_exercises::get_exercises(current_track.title)
                .await
                .unwrap();

            // Acquiring lock after the I/O call is finished is okay.
            // This will not block and happen immediately
            {
                let mut app = app.lock().unwrap();
                app.exercises.add_items(exercises.exercises);
                app.exercises.state.select(Some(0));
            }

            // Change the current window to exercises
            Some(fsm::Window::Exercises)
        }
        TransitionAction::Nop => None,
        TransitionAction::Unselect => None,
    }
}
