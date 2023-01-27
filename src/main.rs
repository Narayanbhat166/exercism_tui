use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table},
    Frame, Terminal,
};

#[derive(Default)]
struct App {
    throbber_state: throbber_widgets_tui::ThrobberState,
}

impl App {
    fn on_tick(&mut self) {
        self.throbber_state.calc_next();
    }
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut app = App::default();
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if let crossterm::event::KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, _app: &mut App) {
    let main_terminal_size = f.size();

    let main_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(main_terminal_size);

    let top_block = Paragraph::new("Welcome to the exercism cli")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    f.render_widget(top_block, main_inner_layout[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_inner_layout[1]);

    let middle_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(middle_chunks[0]);

    let middle_left_tracks_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(middle_left_layout[0]);

    let help_block = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["q", "quit"]),
        Row::new(vec!["↓", "move down"]),
        // You can style the entire row.
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Help").borders(Borders::ALL))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[Constraint::Length(2), Constraint::Length(5)])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");

    f.render_widget(help_block, middle_left_layout[1]);

    let menu_items = [
        ListItem::new("Dashboard"),
        ListItem::new("Your tracks"),
        ListItem::new("All tracks"),
    ];

    let menu_list = List::new(menu_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Menu"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█");

    let tracks_list = List::new([ListItem::new("Select a menu")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tracks"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█");

    let exercises_list = List::new([ListItem::new("Select a track")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Exercises"),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("█");

    let action_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Description");

    f.render_widget(action_block, middle_chunks[1]);

    f.render_widget(menu_list, middle_left_tracks_layout[0]);
    f.render_widget(tracks_list, middle_left_tracks_layout[1]);
    f.render_widget(exercises_list, middle_left_tracks_layout[2]);

    // let loader = throbber_widgets_tui::Throbber::default()
    //     .label("Loading user profile...")
    //     .style(tui::style::Style::default().fg(tui::style::Color::White))
    //     .throbber_style(
    //         tui::style::Style::default().fg(tui::style::Color::White), // .add_modifier(tui::style::Modifier::BOLD),
    //     )
    //     .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
    //     .use_type(throbber_widgets_tui::WhichUse::Spin);

    // Bottom block with all default borders
    let bottom_block = Block::default()
        .borders(Borders::ALL)
        .title("LOGS")
        .border_type(BorderType::Rounded);
    f.render_widget(bottom_block, main_inner_layout[2]);
}
