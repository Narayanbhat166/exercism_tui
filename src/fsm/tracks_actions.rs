use crate::{
    api::{self},
    fsm::{self, TransitionInput},
    App,
};
use crossterm::event::KeyCode;

pub enum TrackActions {
    MoveDown,
    MoveUp,
    SelectTrack,
    GetAllTracks,
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
            TrackActions::GetAllTracks => {
                let all_tracks = api::tracks::get_tracks::get_tracks().await.unwrap();
                app.tracks.add_items(all_tracks.tracks);
                app.tracks.state.select(Some(0));

                None
            }
            TrackActions::SelectTrack => {
                let current_track = app.tracks.get_current_item().unwrap();
                let exercises = api::exercises::get_exercises::get_exercises(current_track.title)
                    .await
                    .unwrap();

                app.exercises.add_items(exercises.exercises);
                app.exercises.state.select(Some(0));

                // Change the current window to exercises
                Some(fsm::Window::Exercises)
            }
            TrackActions::Nop => None,
        }
    }

    pub fn get_action(input: TransitionInput) -> TrackActions {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => Self::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => Self::MoveUp,
            TransitionInput::Key(KeyCode::Right | KeyCode::Enter | KeyCode::Char('l')) => {
                Self::SelectTrack
            }
            TransitionInput::Init => Self::GetAllTracks,
            _ => Self::Nop,
        }
    }
}
