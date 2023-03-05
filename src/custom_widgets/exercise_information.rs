use std::thread::current;

use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, Borders, Row, Table},
};

use crate::App;

pub fn exercise_information(app: &App) -> impl widgets::Widget {
    if let Some(current_exercise) = app.exercises.get_current_item() {
        Table::new(vec![Row::new(vec![
            "Difficulty".to_string(),
            current_exercise.difficulty.to_string(),
        ])])
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(format!("Exercise [{}]", current_exercise.title))
                .borders(Borders::ALL)
                .border_type(widgets::BorderType::Rounded),
        )
        .widths(&[Constraint::Percentage(60), Constraint::Percentage(40)])
        .column_spacing(1)
    } else {
        Table::new(vec![Row::new(vec![
            "Select an exercise to view it's information",
        ])])
        .block(
            Block::default()
                .title("Exercise []")
                .border_type(widgets::BorderType::Rounded)
                .borders(Borders::ALL),
        )
    }
}
