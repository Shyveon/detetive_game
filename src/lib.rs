use std::{rc::Rc, cell::RefCell, io::stdout};

use tui::{backend::CrosstermBackend, Terminal};

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configurando o backend Crossterm para o TUI

    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();
        // Renderizando
        terminal.draw(|rect| ui::draw(rect, &app))?;
        // TODO fazer input funfar aqui em embaixo
    }

    /* 
        Aqui tem que deixar o terminal de volta do jeito que tava antes e fechar a aplicação
    */
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}