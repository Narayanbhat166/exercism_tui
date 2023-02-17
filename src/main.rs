use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crate::{
    layout::divider::layout_divider,
    widgets::{draw_blocks, listblock::StatefulList},
};

mod layout;
mod widgets;

struct App {
    menu_list: widgets::listblock::StatefulList<&'static str>,
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
            menu_list: StatefulList::with_items(vec!["Dashboard", "All tracks", "My tracks"]),
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
                    KeyCode::Up => app.menu_list.previous(),
                    KeyCode::Down => app.menu_list.next(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, _app: &mut App) {
    let main_terminal_size = frame.size();

    let layout = layout_divider(main_terminal_size);
    draw_blocks(frame, layout)

    // f.render_widget(top_block, main_inner_layout[0]);

    // let help_block = Table::new(vec![
    //     // Row can be created from simple strings.
    //     Row::new(vec!["q", "quit"]),
    //     Row::new(vec!["↓", "move down"]),
    //     // You can style the entire row.
    // ])
    // // You can set the style of the entire Table.
    // .style(Style::default().fg(Color::White))
    // // As any other widget, a Table can be wrapped in a Block.
    // .block(Block::default().title("Help").borders(Borders::ALL))
    // // Columns widths are constrained in the same way as Layout...
    // .widths(&[Constraint::Length(2), Constraint::Length(5)])
    // // ...and they can be separated by a fixed spacing.
    // .column_spacing(1)
    // // If you wish to highlight a row in any specific way when it is selected...
    // .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // // ...and potentially show a symbol in front of the selection.
    // .highlight_symbol(">>");

    // f.render_widget(help_block, middle_left_layout[1]);

    // let menu_items = app
    //     .menu_list
    //     .items
    //     .iter()
    //     .map(|item| ListItem::new(item.to_string()))
    //     .collect::<Vec<ListItem>>();

    // let menu_list = List::new(menu_items)
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

    // f.render_stateful_widget(
    //     menu_list,
    //     middle_left_tracks_layout[0],
    //     &mut app.menu_list.state,
    // );

    // let exercises_list =

    // let action_block = ;

    // f.render_widget(action_block, middle_chunks[1]);
    // f.render_widget(tracks_list, middle_left_tracks_layout[1]);
    // f.render_widget(exercises_list, middle_left_tracks_layout[2]);

    // // let loader = throbber_widgets_tui::Throbber::default()
    // //     .label("Loading user profile...")
    // //     .style(tui::style::Style::default().fg(tui::style::Color::White))
    // //     .throbber_style(
    // //         tui::style::Style::default().fg(tui::style::Color::White), // .add_modifier(tui::style::Modifier::BOLD),
    // //     )
    // //     .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
    // //     .use_type(throbber_widgets_tui::WhichUse::Spin);

    // // Bottom block with all default borders

    // f.render_widget(bottom_block, main_inner_layout[2]);
}
