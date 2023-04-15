use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use fsm::{ExecuteTransition, Transition, TransitionInput, Window};
use std::{
    error::Error,
    io,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{
    custom_widgets::{draw_blocks, listblock::StatefulList},
    layout::divider::layout_divider,
};

mod api;
mod async_tasks;
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
}

pub async fn transition(app: Arc<Mutex<App>>, input: TransitionInput) {
    let (transition_action, current_window) = {
        // Those that are not async task, will return immediately,
        // lock can be acquired on them without worries
        let app = app.lock().unwrap();
        let current_window = app.current_window.clone();
        (current_window.get_action(input), current_window)
    };

    // The execute action function will update the state of app and involves i/o
    // The function that requires
    if let Some(new_window) = current_window
        .execute_action(app.clone(), transition_action)
        .await
    {
        let mut app = app.lock().unwrap();
        app.current_window = new_window;
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let app = Arc::new(Mutex::new(App::new()));

    let (sender, receiver) = mpsc::channel::<fsm::TransitionInput>();

    let cloned_app = app.clone();

    let network_calls_thread =
        thread::spawn(move || async_tasks::network_calls_task(cloned_app, receiver));

    let mut last_tick = std::time::Instant::now();

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        terminal.draw(|f| ui(f, app.clone()))?;

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                // Quit on pressing q
                if key_event.code == KeyCode::Char('q') {
                    // Send close message to thread
                    sender.send(fsm::TransitionInput::Quit).unwrap();
                    network_calls_thread.join().unwrap();

                    return Ok(());
                } else {
                    sender
                        .send(fsm::TransitionInput::Key(key_event.code))
                        .unwrap();
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: Arc<Mutex<App>>) {
    let main_terminal_size = frame.size();

    let layout = layout_divider(main_terminal_size);
    draw_blocks(frame, layout, app);
}
