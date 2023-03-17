use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{self, BorderType, Paragraph},
    widgets::{Block, Borders, Wrap},
};

use minimad::{CompositeStyle, Compound, Line};

use crate::{App};

fn transform_md_line(element: Compound, md_style: CompositeStyle) -> Span {
    let style = match md_style {
        CompositeStyle::Header(_header_strength) => Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
        CompositeStyle::Code => Style::default().bg(Color::LightYellow).fg(Color::LightRed),
        _ => Style::default(),
    };

    let line_with_break = format!("{}\n\n", element.src);

    Span::styled(line_with_break, style)
}

fn parse_markdown(md_string: &String) -> tui::text::Text {
    let parsed_md = minimad::parse_text(&md_string, minimad::Options::default());
    let mut resulting_text = vec![];
    for line in parsed_md.lines {
        match line {
            Line::Normal(line_data) => {
                let style = line_data.style;
                let data = line_data.compounds;

                let first_element = data.first();
                match first_element {
                    Some(ele) => resulting_text.push(transform_md_line(ele.to_owned(), style)),
                    None => resulting_text.push(Span {
                        content: "\n".into(),
                        style: Style::default(),
                    }),
                }
            }
            _ => {}
        }
    }

    tui::text::Text::from(Spans::from(resulting_text))
}

pub fn description(app: &App) -> impl widgets::Widget + '_ {
    let transformed_text = parse_markdown(&app.description);
    Paragraph::new(transformed_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}
