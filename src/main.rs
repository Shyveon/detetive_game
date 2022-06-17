pub mod ui;
pub mod consts;
pub mod menu;

use std::{io::Error, io::stdout, time::Duration};
use menu::MenuItem;
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture, poll, read, Event, KeyModifiers, KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use ui::ui;

fn main() -> Result<(), Error> {

    /*
        Aqui estamos preparando o Terminal
    */
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut active_menu_item: usize = MenuItem::Historia.into();

    /*
        Aqui abaixo vamos começar a desenhar na tela
    */
    loop {
        terminal.draw(|f| ui(f, active_menu_item))?;
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    if event.code.eq(&KeyCode::Char('c')) && event.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }

                    if event.code.eq(&KeyCode::Char('h')) {
                        active_menu_item = MenuItem::Historia.into();
                    }

                    if event.code.eq(&KeyCode::Char('d')) {
                        active_menu_item = MenuItem::Dicas.into();
                    }

                    if event.code.eq(&KeyCode::Char('p')) {
                        active_menu_item = MenuItem::Portas.into();
                    }

                    if active_menu_item.eq(&MenuItem::Portas.into()) {
                        match event.code {
                            KeyCode::Char('v') => active_menu_item = MenuItem::Morte.into(),
                            KeyCode::Char('a') => active_menu_item = MenuItem::Vitoria.into(),
                            KeyCode::Char('m') => active_menu_item = MenuItem::Morte.into(),
                            KeyCode::Char('e') => active_menu_item = MenuItem::Morte.into(),
                            _ => ()
                        }
                    }
                },
                Event::Mouse(event) => (),
                Event::Resize(width, height) => (),
            }
        }
    }

    /* 
        Restaura o terminal
    */
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}