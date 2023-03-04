use crossterm::event::KeyCode;

use crate::App;
pub mod exercises_action;
pub mod tracks_actions;

pub enum Window {
    Tracks,
    Exercises,
    BottomBar,
    Description,
    Help,
    SortAndFilter,
}

pub enum TransitionInput {
    Key(KeyCode),
    Init,
}

pub struct StateMachine {
    current_state: Window,
}

impl StateMachine {
    pub async fn transition(&mut self, input: TransitionInput, app: &mut App) {
        match self.current_state {
            Window::Tracks => {
                let action = tracks_actions::TrackActions::get_action(input);
                let state_change = action.execute_action(app).await;
                if let Some(new_state) = state_change {
                    self.current_state = new_state
                }
            }
            Window::Exercises => {
                let action = exercises_action::ExercisesAction::get_action(input);
                let state_change = action.execute_action(app).await;
                if let Some(new_state) = state_change {
                    self.current_state = new_state
                }
            }
            Window::BottomBar => todo!(),
            Window::Description => todo!(),
            Window::Help => todo!(),
            Window::SortAndFilter => todo!(),
        }
    }

    pub fn new() -> Self {
        Self {
            current_state: Window::Tracks,
        }
    }
}
