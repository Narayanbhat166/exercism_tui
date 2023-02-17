

use tui::{
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, BorderType, Borders, List, ListItem},
};

/// Tracks that are available and chosen
/// Differenciate between available and chosen
/// TODO: provide a filter for the above functionality
pub fn tracks() -> impl widgets::Widget {
    List::new([ListItem::new("Tracks available")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tracks"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█")
}
