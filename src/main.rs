use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::{
    layout::divider::layout_divider,
    widgets::{draw_blocks, listblock::StatefulList},
};

mod api;
mod layout;
mod widgets;

pub struct App {
    tracks: widgets::listblock::StatefulList<String>,
    exercises: widgets::listblock::StatefulList<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
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
            tracks: StatefulList::with_items(vec!["Rust".to_string(), "Cpp".to_string()]),
            exercises: StatefulList::with_items(vec![]),
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut app = App::new();
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

    // let tracks = vec![
    //     ListItem::new("Rust".to_string()),
    //     ListItem::new("Cpp".to_string()),
    // ];

    // let tracks_list = List::new(tracks)
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded)
    //             .title("Menu"),
    //     )
    //     .style(Style::default().fg(Color::White))
    //     .highlight_style(
    //         Style::default()
    //             .add_modifier(Modifier::ITALIC)
    //             .fg(Color::Cyan),
    //     )
    //     .highlight_symbol("█ ");

    // frame.render_stateful_widget(tracks_list, layout.tracks, &mut app.tracks.state);
}
