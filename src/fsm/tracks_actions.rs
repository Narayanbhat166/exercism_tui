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

pub enum TrackActions {
    MoveDown,
    MoveUp,
    SelectTrack,
    Nop,
}

impl TrackActions {
    pub async fn execute_action(&self, app: &mut App) -> Option<fsm::Window> {
        match self {
            TrackActions::MoveDown => {
                app.tracks.next();
                None
            }
            TrackActions::MoveUp => {
                app.tracks.previous();
                None
            }
            TrackActions::SelectTrack => {
                let current_track = app.tracks.get_current_item().unwrap();
                let exercises = api::exercises::get_exercises::get_exercises(current_track.title)
                    .await
                    .unwrap();

                app.exercises.add_items(exercises.exercises);
                app.tracks.unselect();

                Some(fsm::Window::Exercises)
            }
            TrackActions::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> TrackActions {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => TrackActions::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => TrackActions::MoveUp,
            TransitionInput::Key(KeyCode::Right) => TrackActions::SelectTrack,
            _ => TrackActions::Nop,
        }
    }
}
