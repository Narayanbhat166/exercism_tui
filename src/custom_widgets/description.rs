use tui::{
    style::{self, Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{self, BorderType, Paragraph},
    widgets::{Block, Borders, Wrap},
};

use minimad::{Composite, CompositeStyle, Compound, Line};

use crate::App;
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
    // let transformed_text = tui::text::Text::styled("Hola `Hello`", style::Style::default());
    app.description.current_height = u16::try_from(transformed_text.height())
        .expect("Failed when converting usize to u16, overflow");
    Paragraph::new(transformed_text)
        .wrap(Wrap { trim: true })
        .scroll(app.description.scroll_offset)
        .block(
            Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}
