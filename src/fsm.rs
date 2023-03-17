use crossterm::{
    event::{KeyCode},
};








pub mod exercises_action;
pub mod tracks_actions;

#[derive(PartialEq)]
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
