use std::time::{Duration, Instant};

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Enter Duration (seconds)"),
    );
    let app_result = run(&mut terminal, &mut textarea);
    ratatui::restore();
    app_result
}

enum TimerState {
    Idle,
    Running { start: Instant, duration: Duration },
    Finished,
}

fn run(terminal: &mut DefaultTerminal, textarea: &mut TextArea) -> Result<()> {
    let mut timer_state = TimerState::Idle;

    loop {
        terminal.draw(|frame| draw(frame, textarea, &timer_state))?;

        match handle_events(textarea, &mut timer_state)? {
            Some(AppEvent::Quit) => break,
            Some(AppEvent::InputHandled) | None => {}
        }
    }
    Ok(())
}

enum AppEvent {
    Quit,
    InputHandled,
}

fn handle_events(
    textarea: &mut TextArea,
    timer_state: &mut TimerState,
) -> Result<Option<AppEvent>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if textarea.input(key) {
                return Ok(Some(AppEvent::InputHandled));
            }

            match key.code {
                KeyCode::Char('q') => return Ok(Some(AppEvent::Quit)),
                KeyCode::Enter => {
                    let input_text = textarea.lines().join("");
                    if let Ok(seconds) = input_text.trim().parse::<u64>() {
                        *timer_state = TimerState::Running {
                            start: Instant::now(),
                            duration: Duration::from_secs(seconds),
                        };
                        textarea.delete_line_by_head();
                        textarea.delete_line_by_end();
                    }
                }
                KeyCode::Char('r') => {
                    *timer_state = TimerState::Idle;
                }
                _ => {}
            }
        }
    }

    // Update timer state
    if let TimerState::Running { start, duration } = timer_state {
        if start.elapsed() >= *duration {
            *timer_state = TimerState::Finished;
        }
    }

    Ok(None)
}

fn draw(frame: &mut Frame, textarea: &mut TextArea, timer_state: &TimerState) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(frame.area());

    let greeting = Paragraph::new("How long would you like to focus?");
    frame.render_widget(greeting, chunks[0]);

    // Display timer
    let (timer_text, style) = match timer_state {
        TimerState::Idle => ("Ready to start...".to_string(), Style::default()),
        TimerState::Running { start, duration } => {
            let elapsed = start.elapsed();
            let remaining = duration.saturating_sub(elapsed);
            let secs = remaining.as_secs();
            let mins = secs / 60;
            let secs = secs % 60;
            (
                format!("Time remaining: {:02}:{:02}", mins, secs),
                Style::default().fg(Color::Green),
            )
        }
        TimerState::Finished => (
            "Time's up! Press 'r' to reset".to_string(),
            Style::default().fg(Color::Red),
        ),
    };

    let timer_widget = Paragraph::new(timer_text)
        .style(style)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(timer_widget, chunks[1]);

    frame.render_widget(textarea.widget(), chunks[2]);
}
