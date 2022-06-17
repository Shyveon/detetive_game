use std::vec;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph, BorderType, Tabs, Wrap},
    Frame,
    style::{
        Color,
        Style, Modifier,
    }, text::{Spans, Span, Text},
};

use crate::{consts::{DICAS, HISTORIA, PORTA}, menu::MenuItem};

pub fn ui<B: Backend>(f: &mut Frame<B>, active_menu_item: usize) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());

    let titulos_menus = vec!["História", "Dicas", "Portas"];

    let menu = titulos_menus
        .iter()
        .map(|t| {
            let (primeiro, resto) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    primeiro,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(resto, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(active_menu_item)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .border_type(BorderType::Thick)
                .style(Style::default().bg(Color::Black))
        );

    let dicas = Paragraph::new(DICAS.join("\n"))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::all())
                .style(Style::default().bg(Color::Black))
                .title("Dicas")
                .border_type(BorderType::Thick),
        );

    let historia = Paragraph::new(HISTORIA)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::all())
                .style(Style::default().bg(Color::Black))
                .title("Historia")
                .border_type(BorderType::Thick),
        );

    let mut spans: Vec<Spans> = PORTA.split('\n').into_iter().map(|f| {
        Spans::from(vec![
                Span::styled(f, Style::default().fg(Color::Red)),
                Span::raw("    "),
                Span::styled(f, Style::default().fg(Color::Blue)),
                Span::raw("    "),
                Span::styled(f, Style::default().fg(Color::Yellow)),
                Span::raw("    "),
                Span::styled(f, Style::default().fg(Color::Green)),
            ])
    }).collect();

    let mut letters: Vec<Spans> = vec![
        Spans::from(vec![
                Span::styled("V", Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED)), 
                Span::styled("ermelho", Style::default().fg(Color::Green)),
                Span::raw("        "),
                Span::styled("A", Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED)),
                Span::styled("zul", Style::default().fg(Color::Green)),
                Span::raw("        "),
                Span::styled("A", Style::default().fg(Color::Green)),
                Span::styled("m", Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED)),
                Span::styled("arelo", Style::default().fg(Color::Green)),
                Span::raw("        "),
                Span::styled("V", Style::default().fg(Color::Green)),
                Span::styled("e", Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED)),
                Span::styled("rde", Style::default().fg(Color::Green))
            ]),
    ];

    spans.append(&mut letters);

    let game = Paragraph::new(spans)
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black).fg(Color::Green))
                .title("Portas")
                .border_type(BorderType::Thick),
        );

    let loss = Paragraph::new("Você perdeu :(")
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black).fg(Color::Green))
                .title("Morte")
                .border_type(BorderType::Thick)
        );

    let win = Paragraph::new("Você ganhou :)")
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black).fg(Color::Green))
                .title("Vitória!")
                .border_type(BorderType::Thick)
        );

    f.render_widget(tabs, chunks[0]);
    match active_menu_item {
        0 => f.render_widget(historia, chunks[1]),
        1 => f.render_widget(dicas, chunks[1]),
        2 => f.render_widget(game, chunks[1]),
        3 => f.render_widget(loss, chunks[1]),
        4 => f.render_widget(win, chunks[1]),
        _ => f.render_widget(dicas, chunks[1])
    }
}