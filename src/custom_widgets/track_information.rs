use std::sync::Mutex;

use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets,
    widgets::{Block, Borders, Row, Table},
};

use crate::App;

pub fn track_information(app: &Mutex<App>) -> impl widgets::Widget {
    let app = app.lock().unwrap();
    if let Some(current_track) = app.tracks.get_current_item() {
        Table::new(vec![
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
        .block(
            Block::default()
                .title(format!("Track [{}]", current_track.title))
                .border_type(widgets::BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .widths(&[Constraint::Percentage(60), Constraint::Percentage(40)])
        .column_spacing(1)
    } else {
        Table::new(vec![Row::new(vec![
            "Select a track to view it's information",
        ])])
        .block(Block::default().title("Track []"))
    }
}
