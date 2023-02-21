use tui::{
    style::{Color, Modifier, Style},
    widgets::{self, ListState},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::App;

/// Tracks that are available and chosen
/// Differenciate between available and chosen
/// TODO: provide a filter for the above functionality, and sort
pub fn tracks(app: &App) -> impl widgets::StatefulWidget<State = ListState> {
    let track_titles = app
        .tracks
        .items
        .iter()
        .map(|track| ListItem::new(track.title.to_owned()))
        .collect::<Vec<_>>();

    List::new(track_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Tracks"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Cyan),
        )
        .highlight_symbol("█ ")
}
