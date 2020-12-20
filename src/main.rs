mod event;

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use event::{Event, Events};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Hello")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);

            let layout = Layout::default()
                            .margin(15)
                            .constraints(
                                [
                                Constraint::Percentage(100)
                                ].as_ref(),
                            )
                            .split(size);
    
            let paragraph = Paragraph::new("Hello, World!")
                            .style(Style::default())
                            .alignment(tui::layout::Alignment::Center);

            f.render_widget(paragraph, layout[0]);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    };
    Ok(())
}
