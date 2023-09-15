use std::io;
use crossterm:: {terminal::{EnterAlternateScreen, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture, KeyCode}};
use crossterm::terminal;

fn main() -> Result<(), E> {
    /* 1- Initialize the terminal */
    let backend = tui::backend::CrosstermBackend::new(std::io::stderr());
    let mut terminal = tui::Terminal::new(backend)?;

    /* 2- Create an event handler */
    mod event;

    let event_handler = event::EventHandler::new(250);

    /* 3- Prepare the terminal for rendering */

    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    
    /* 4- Start the render loop */
    let mut running = true;
    while running {

        /* 4.1- Render widgets */
        terminal.draw(|frame| {
            frame.render_widget(tui::widgets::Paragraph::new("Hello World!"), frame.size())
        })?;

        /* 4.2- Handle events */
        match event_handler.next()? {
            Event::Key(key_event) => match key_event.code {
                // exit on ESC key press
                KeyCode::Esc => {
                    running = false,
                }
                _ => {}
            },
            _ => {}
        }
    }

    /* 5- Flush the terminal before exit */
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
