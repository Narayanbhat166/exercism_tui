use std::thread::current;

use crate::{
    api::{
        self,
        models::{Exercise, Track},
    },
    custom_widgets::listblock::StatefulList,
    fsm::{self, TransitionInput, Window},
    App,
};
use crossterm::event::KeyCode;

pub enum ExercisesAction {
    MoveDown,
    MoveUp,
    SelectExercise,
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

                let description = api::description::get_description::get_description(
                    current_track.slug,
                    current_exercise.slug,
                )
                .await
                .unwrap();

                app.description = description;

                Some(Window::Description)
            }
            ExercisesAction::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> Self {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => Self::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => Self::MoveUp,
            TransitionInput::Key(KeyCode::Char('l') | KeyCode::Enter) => Self::SelectExercise,
            _ => Self::Nop,
        }
    }
}
