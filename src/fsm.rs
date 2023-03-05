use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{
    custom_widgets::{draw_blocks, listblock::StatefulList},
    layout::divider::layout_divider,
};

use crate::{api, custom_widgets};

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
