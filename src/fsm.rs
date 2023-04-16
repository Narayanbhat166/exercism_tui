use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use crossterm::event::KeyCode;

use crate::App;

use self::{
    description_action::execute_desctiption_action, exercises_action::execute_exercise_action,
    tracks_actions::execute_track_action,
};

pub mod description_action;
pub mod exercises_action;
pub mod tracks_actions;

// This trait should be applied on a window
pub trait Transition {
    fn get_action(&self, input: TransitionInput) -> TransitionAction;
}

#[async_trait]
pub trait ExecuteTransition {
    async fn execute_action(
        &self,
        app: Arc<Mutex<App>>,
        action: TransitionAction,
    ) -> Option<Window>;
}

#[derive(PartialEq, Clone)]
pub enum Window {
    Tracks,
    Exercises,
    BottomBar,
    Description,
    Help,
    SortAndFilter,
}

#[derive(PartialEq)]
pub enum TransitionInput {
    Key(KeyCode),
    Init,
    Quit,
}

pub enum TransitionAction {
    MoveDown,
    MoveUp,
    Select,
    Unselect,
    Nop,
    Init,
}

impl Transition for Window {
    fn get_action(&self, input: TransitionInput) -> TransitionAction {
        match input {
            TransitionInput::Key(KeyCode::Down | KeyCode::Char('j')) => TransitionAction::MoveDown,
            TransitionInput::Key(KeyCode::Up | KeyCode::Char('k')) => TransitionAction::MoveUp,
            TransitionInput::Key(KeyCode::Right | KeyCode::Enter | KeyCode::Char('l')) => {
                TransitionAction::Select
            }
            TransitionInput::Key(KeyCode::Left | KeyCode::Char('h')) => TransitionAction::Unselect,
            TransitionInput::Init => TransitionAction::Init,
            _ => TransitionAction::Nop,
        }
    }
}

#[async_trait]
impl ExecuteTransition for Window {
    async fn execute_action(
        &self,
        app: Arc<Mutex<App>>,
        action: TransitionAction,
    ) -> Option<Window> {
        match self {
            Window::Tracks => execute_track_action(app, action).await,
            Window::Exercises => execute_exercise_action(app, action).await,
            Window::BottomBar => todo!(),
            Window::Description => execute_desctiption_action(app, action).await,
            Window::Help => todo!(),
            Window::SortAndFilter => todo!(),
        }
    }
}
