use std::sync::Mutex;

use tui::{
    style::{Color, Modifier, Style},
    widgets::{self, ListState},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{fsm::Window, App};

/// Exercises that are available and chosen
/// Differenciate between done and not done
/// TODO: provide a filter for the above functionality
pub fn exercises(app: &Mutex<App>) -> impl widgets::StatefulWidget<State = ListState> {
    let app = app.lock().unwrap();
    // If current window is selected, then use different color to indicate that it is selected
    let (border_style, highlight_symbol) = if app.current_window == Window::Exercises {
        (Style::default().fg(Color::Cyan), "█ ")
    } else {
        (Style::default().fg(Color::Gray), "  ")
    };
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
                .border_style(border_style)
                .title("Exercises"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Cyan),
        )
        .highlight_symbol(highlight_symbol)
}
