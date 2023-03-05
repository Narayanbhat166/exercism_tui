use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{self, BorderType},
    widgets::{Block, Borders, Row, Table},
};

use crate::{api::models, App};

pub fn description(app: &App) -> impl widgets::Widget {
    Block::default()
        .title("Description")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}
