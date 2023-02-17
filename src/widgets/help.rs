

use tui::{
    layout::{Constraint},
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, Borders, Row, Table},
};

/// The help menu shown should be relative to the active menu
/// TODO: use an enum for knowing the menu and display appropriate help commands
pub fn help_table() -> impl widgets::Widget {
    Table::new(vec![
        Row::new(vec!["q", "quit"]),
        Row::new(vec!["↓", "move down"]),
    ])
    .style(Style::default().fg(Color::White))
    .block(Block::default().title("Help").borders(Borders::ALL))
    .widths(&[Constraint::Length(2), Constraint::Length(5)])
    .column_spacing(1)
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}
