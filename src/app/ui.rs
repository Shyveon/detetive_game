pub fn draw<B>(rect: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let size = rect.size();
    // TODO Verificar o tamanho
    
    // Layout vertical
    let chuncks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);
    
    // Bloco com o título
    let title = draw_title();
    rect.render_widget(title, chuncks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Jogo detetive")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}