use std::thread::current;

use crate::{
    api::{
        self,
        models::{Exercise, Track},
    },
    custom_widgets::listblock::StatefulList,
    fsm::{self, TransitionInput},
    App,
};
use crossterm::event::KeyCode;

pub enum ExercisesAction {
    MoveDown,
    MoveUp,
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
            ExercisesAction::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> Self {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => Self::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => Self::MoveUp,
            _ => Self::Nop,
        }
    }
}
