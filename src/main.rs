use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{
    custom_widgets::{draw_blocks, listblock::StatefulList},
    layout::divider::layout_divider,
};

mod api;
mod custom_widgets;
mod layout;

pub struct App {
    tracks: custom_widgets::listblock::StatefulList<api::models::Track>,
    exercises: custom_widgets::listblock::StatefulList<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

impl App {
    fn new() -> App {
        App {
            tracks: StatefulList::new(),
            exercises: StatefulList::with_items(vec![]),
        }
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut app = App::new();
    let tracks = api::tracks::get_tracks::get_tracks().await.unwrap();
    app.tracks.add_items(tracks.tracks);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.tracks.previous(),
                    KeyCode::Down => app.tracks.next(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let main_terminal_size = frame.size();

    let layout = layout_divider(main_terminal_size);
    draw_blocks(frame, layout, app);
}
