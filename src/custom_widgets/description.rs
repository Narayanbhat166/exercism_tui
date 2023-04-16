use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{self, BorderType, Paragraph},
    widgets::{Block, Borders, Wrap},
};

use minimad::{Composite, CompositeStyle, Compound, Line};

use crate::{fsm::Window, App};
enum CompoundType {
    Code,
    NormalString,
}
fn get_compound_type(compound: &Compound) -> CompoundType {
    if compound.code {
        CompoundType::Code
    } else {
        CompoundType::NormalString
    }
}

fn transform_md_line(element: Composite) -> tui::text::Text {
    let style = match element.style {
        CompositeStyle::Header(_header_strength) => Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
        CompositeStyle::Code => Style::default().bg(Color::LightYellow).fg(Color::LightRed),
        _ => Style::default(),
    };

    let spans = element
        .compounds
        .iter()
        .map(|compound| match get_compound_type(&compound) {
            CompoundType::Code => Span::styled(
                compound.src,
                style.add_modifier(Modifier::BOLD).fg(Color::Gray),
            ),
            CompoundType::NormalString => Span::styled(compound.src, style),
        })
        .collect::<Vec<_>>();

    tui::text::Text::from(Spans::from(spans))
}

pub fn parse_markdown(md_string: &String) -> tui::text::Text {
    let parsed_md = minimad::parse_text(&md_string, minimad::Options::default());
    let mut resulting_text = tui::text::Text::default();
    for line in parsed_md.lines {
        match line {
            Line::Normal(line_data) => {
                resulting_text.extend(transform_md_line(line_data));
            }
            _ => {}
        }
    }

    resulting_text
}

pub fn description(app: &mut App) -> impl widgets::Widget + '_ {
    let transformed_text = parse_markdown(&app.description.text);
    app.description.current_height = u16::try_from(transformed_text.height())
        .expect("Failed when converting usize to u16, overflow");

    let border_style = if app.current_window == Window::Description {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let current_track = app.tracks.get_current_item();
    let current_exercise = app.exercises.get_current_item();

    let description_title = if app.current_window == Window::Description {
        current_track
            .zip(current_exercise)
            .map(|(track, exercise)| {
                format!(
                    "Description Track: [{}] Exercise: [{}]",
                    track.title, exercise.title
                )
            })
            .unwrap_or("Description".to_string())
    } else {
        "Description".to_string()
    };

    Paragraph::new(transformed_text)
        .wrap(Wrap { trim: true })
        .scroll(app.description.scroll_offset)
        .block(
            Block::default()
                .title(description_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        )
}
