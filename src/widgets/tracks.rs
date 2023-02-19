use tui::{
    style::{Color, Modifier, Style},
    widgets::{self, ListState},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::App;

/// Tracks that are available and chosen
/// Differenciate between available and chosen
/// TODO: provide a filter for the above functionality
pub fn tracks() -> impl widgets::StatefulWidget<State = ListState> {
    let tracks = vec![
        ListItem::new("Rust".to_string()),
        ListItem::new("Cpp".to_string()),
    ];

    List::new(tracks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Menu"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Cyan),
        )
        .highlight_symbol("█ ")

    // List::new([ListItem::new("Tracks available")])
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded)
    //             .title("Tracks"),
    //     )
    //     .style(Style::default().fg(Color::White))
    //     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    //     .highlight_symbol("█")
}
