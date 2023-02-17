

use tui::{
    widgets,
    widgets::{Block, BorderType, Borders},
};

/// This bar will display any background actvity going on
pub fn bottom_bar() -> impl widgets::Widget {
    Block::default()
        .borders(Borders::ALL)
        .title("LOGS")
        .border_type(BorderType::Rounded)
}
