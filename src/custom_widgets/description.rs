use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{self, BorderType, Paragraph},
    widgets::{Block, Borders, Row, Table},
};

use crate::{api::models, App};

pub fn description(app: &App) -> impl widgets::Widget {
    Paragraph::new(Text::from(app.description.clone())).block(
        Block::default()
            .title("Description")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}
