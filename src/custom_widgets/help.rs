use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, Borders, Row, Table},
};

/// The help menu shown should be relative to the active menu
/// TODO: use an enum for knowing the menu and display appropriate help commands
pub fn help_table() -> impl widgets::Widget {
    Table::new(vec![
        Row::new(vec!["q", "quit"]),
        Row::new(vec!["↓ / j", "move down"]),
        Row::new(vec!["↑ / i", "move down"]),
    ])
    .style(Style::default().fg(Color::White))
    .block(Block::default().title("Help").borders(Borders::ALL))
    .widths(&[Constraint::Length(10), Constraint::Length(10)])
    .column_spacing(1)
    .header(Row::new(vec!["Key", "Action"]))
}
