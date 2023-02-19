use tui::{
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, BorderType, Borders, List, ListItem},
};

/// Exercises that are available and chosen
/// Differenciate between done and not done
/// TODO: provide a filter for the above functionality
pub fn exercises() -> impl widgets::Widget {
    List::new([ListItem::new("Select a track")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Exercises"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█")
}
