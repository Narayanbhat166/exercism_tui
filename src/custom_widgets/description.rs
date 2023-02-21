use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, Borders, Row, Table},
};

use crate::App;

pub fn description(app: &App) -> impl widgets::Widget {
    let current_track = app.tracks.get_current_item().unwrap();
    Table::new(vec![
        Row::new(vec!["Title".to_string(), current_track.title]),
        Row::new(vec![
            "Number of concepts".to_string(),
            current_track.num_concepts.to_string(),
        ]),
        Row::new(vec![
            "Number of exercises".to_string(),
            current_track.num_exercises.to_string(),
        ]),
    ])
    .style(Style::default().fg(Color::White))
    .block(Block::default().title("Description").borders(Borders::ALL))
    .widths(&[Constraint::Length(20), Constraint::Length(5)])
    .column_spacing(2)
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}
