use std::sync::Mutex;

use tui::widgets;

use throbber_widgets_tui;

use crate::App;

/// This bar will display any background actvity going on
/// This will display something only if there is some network activity happening
pub fn bottom_bar(app: &Mutex<App>) -> Option<impl widgets::Widget> {
    let app = app.lock().unwrap();
    app.bottom_bar
        .as_ref()
        .map(|logs_string| throbber_widgets_tui::Throbber::default().label(logs_string.to_string()))
}
