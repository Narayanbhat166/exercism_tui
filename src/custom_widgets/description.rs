use tui::{
    style::{Color, Modifier, Style, self},
    text::{Span, Spans},
    widgets::{self, BorderType, Paragraph},
    widgets::{Block, Borders, Wrap},
};

use minimad::{CompositeStyle, Compound, Line, Composite};

use crate::{App};
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

    let spans = element.compounds.iter().map(|compound| {
        match get_compound_type(&compound) {
            CompoundType::Code => Span::styled(compound.src, style.add_modifier(Modifier::BOLD).fg(Color::Gray)),
            CompoundType::NormalString => Span::styled(compound.src, style),
        }
    }).collect::<Vec<_>>();

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

pub fn description(app: &App) -> impl widgets::Widget + '_ {
    let transformed_text = parse_markdown(&app.description);
    // let transformed_text = tui::text::Text::styled("Hola `Hello`", style::Style::default());
    Paragraph::new(transformed_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}
