


use tui::{
    widgets,
    widgets::{Block, BorderType, Borders},
};

pub fn dashboard() -> impl widgets::Widget {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Description")
}
