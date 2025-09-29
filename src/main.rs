//! # [Ratatui] Hello World example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// This example does not handle events or update the application state. It just draws a greeting
/// and exits when the user presses 'q'.
fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let mut terminal = ratatui::init(); // Initialize the terminal, handle `?`
    let mut textarea = TextArea::default();
    textarea.set_block(Block::default().borders(Borders::ALL).title("Enter Duration"));
    let app_result = run(&mut terminal, &mut textarea).context("app loop failed"); // Pass terminal and textarea
    ratatui::restore(); 
    app_result
}


fn run(terminal: &mut DefaultTerminal, textarea: &mut TextArea) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, textarea))?;

        if let Some(event) = handle_events(textarea)? {
            match event {
                AppEvent::Quit => break,
                AppEvent::InputHandled => {} 
            }
        }
    }
    Ok(())
}

enum AppEvent {
    Quit,
    InputHandled,
}


fn handle_events(textarea: &mut TextArea) -> Result<Option<AppEvent>> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            // Let textarea handle most key inputs for editing
            if textarea.input(key) {
                return Ok(Some(AppEvent::InputHandled));
            }
            if KeyCode::Char('q') == key.code {
                return Ok(Some(AppEvent::Quit));
            }
            if KeyCode::Enter == key.code {
                let input_text = textarea.lines().join("\n");
                println!("User entered: {}", input_text);
                textarea.delete_line_by_head(); 
                textarea.delete_line_by_end();
            }
        }
    }
    Ok(None)
}


fn draw(frame: &mut Frame, textarea: &mut TextArea) {
    let area = frame.area();
    let chunks = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Length(3), // For the greeting
        ratatui::layout::Constraint::Min(0),    // For the textarea
    ])
    .split(area);

    let greeting = Paragraph::new("How long would you like to focus?");
    frame.render_widget(greeting, chunks[0]); // Render greeting in the top chunk

    frame.render_widget(textarea.widget(), chunks[1]);
}