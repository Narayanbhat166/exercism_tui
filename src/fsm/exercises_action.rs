use crate::{
    api::{self},
    fsm::{self, Window},
    App,
};

use super::TransitionAction;

pub async fn execute_exercise_action(
    app: &mut App,
    action: TransitionAction,
) -> Option<fsm::Window> {
    match action {
        TransitionAction::MoveDown => {
            app.exercises.next();
            None
        }
        TransitionAction::MoveUp => {
            app.exercises.previous();
            None
        }
        TransitionAction::Select => {
            let current_track = app.tracks.get_current_item().unwrap();
            let current_exercise = app.exercises.get_current_item().unwrap();

            let description_text = api::description::get_description::get_description(
                current_track.slug,
                current_exercise.slug,
            )
            .await
            .unwrap();

            app.description.text = description_text;

            Some(Window::Description)
        }
        TransitionAction::Unselect => Some(Window::Tracks),
        TransitionAction::Nop | TransitionAction::Init => None,
    }
}
