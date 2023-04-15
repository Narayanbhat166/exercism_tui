use tui::{
    widgets,
    widgets::{Block, BorderType, Borders},
};

use throbber_widgets_tui;

/// This bar will display any background actvity going on
pub fn bottom_bar() -> impl widgets::Widget {
    let simple = throbber_widgets_tui::Throbber::default();

    // Block::default()
    //     .borders(Borders::ALL)
    //     .title("LOGS")
    //     .border_type(BorderType::Rounded)

    simple
}
