use std::sync::Mutex;

use tui::{
    style::{Color, Modifier, Style},
    widgets::{self, ListState},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::App;

/// Exercises that are available and chosen
/// Differenciate between done and not done
/// TODO: provide a filter for the above functionality
pub fn exercises(app: &Mutex<App>) -> impl widgets::StatefulWidget<State = ListState> {
    let app = app.lock().unwrap();
    let list_items = if app.exercises.items.len() == 0 {
        vec![ListItem::new("Select a track")]
    } else {
        app.exercises
            .items
            .iter()
            .map(|item| ListItem::new(item.title.to_owned()))
            .collect::<Vec<_>>()
    };

    List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Exercises"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Cyan),
        )
        .highlight_symbol("█ ")
}
