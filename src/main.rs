use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use fsm::{
    description_action, exercises_action, tracks_actions, ExecuteTransition, Transition,
    TransitionInput, Window,
};
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
mod fsm;
mod layout;

pub struct DescriptionData {
    text: String,
    scroll_offset: (u16, u16),
    pub max_height: u16,
    pub current_height: u16,
}

impl DescriptionData {
    fn new() -> Self {
        Self {
            text: String::new(),
            scroll_offset: (0, 0),
            max_height: 0,
            current_height: 0,
        }
    }
}

pub struct App {
    current_window: fsm::Window,
    tracks: custom_widgets::listblock::StatefulList<api::models::Track>,
    exercises: custom_widgets::listblock::StatefulList<api::models::Exercise>,
    description: DescriptionData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the exercism token
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
            current_window: Window::Tracks,
            tracks: StatefulList::new(),
            exercises: StatefulList::new(),
            description: DescriptionData::new(),
        }
    }

    pub async fn transition(&mut self, input: TransitionInput) {
        let current_window = &self.current_window.clone();
        let transition_action = current_window.get_action(input);
        if let Some(new_window) = current_window.execute_action(self, transition_action).await {
            self.current_window = new_window;
        }
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut app = App::new();
    // Send init event so that the current window can be initialized
    app.transition(fsm::TransitionInput::Init).await;

    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                // Quit on pressing q
                if key_event.code == KeyCode::Char('q') {
                    return Ok(());
                } else {
                    app.transition(fsm::TransitionInput::Key(key_event.code))
                        .await
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
