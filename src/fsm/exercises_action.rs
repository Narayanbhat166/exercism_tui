use crate::{
    api::{self},
    fsm::{self, TransitionInput, Window},
    App,
};
use crossterm::event::KeyCode;

pub enum ExercisesAction {
    MoveDown,
    MoveUp,
    SelectExercise,
    UnselectExercise,
    Nop,
}

impl ExercisesAction {
    pub async fn execute_action(&self, app: &mut App) -> Option<fsm::Window> {
        match self {
            ExercisesAction::MoveDown => {
                app.exercises.next();
                None
            }
            ExercisesAction::MoveUp => {
                app.exercises.previous();
                None
            }
            ExercisesAction::SelectExercise => {
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
            ExercisesAction::UnselectExercise => Some(Window::Tracks),
            ExercisesAction::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> Self {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => Self::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => Self::MoveUp,
            TransitionInput::Key(KeyCode::Char('l') | KeyCode::Enter) => Self::SelectExercise,
            TransitionInput::Key(KeyCode::Char('h')) => Self::UnselectExercise,
            _ => Self::Nop,
        }
    }
}
