use crate::{api, fsm, App};

use super::{TransitionAction, Window};

pub async fn execute_track_action(app: &mut App, action: TransitionAction) -> Option<Window> {
    match action {
        TransitionAction::MoveDown => {
            app.tracks.next();
            None
        }
        TransitionAction::MoveUp => {
            app.tracks.previous();
            None
        }
        TransitionAction::Init => {
            let all_tracks = api::tracks::get_tracks::get_tracks().await.unwrap();
            app.tracks.add_items(all_tracks.tracks);
            app.tracks.state.select(Some(0));

            None
        }
        TransitionAction::Select => {
            let current_track = app.tracks.get_current_item().unwrap();
            let exercises = api::exercises::get_exercises::get_exercises(current_track.title)
                .await
                .unwrap();

            app.exercises.add_items(exercises.exercises);
            app.exercises.state.select(Some(0));

            // Change the current window to exercises
            Some(fsm::Window::Exercises)
        }
        TransitionAction::Nop => None,
        TransitionAction::Unselect => None,
    }
}
