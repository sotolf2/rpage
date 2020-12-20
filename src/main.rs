mod event;

use std::{error::Error, io};
use std::fs::read_to_string;
use std::env;
use termion::{event::Key, raw::IntoRawMode};
use event::{Event, Events};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    style::Style,
    Terminal,
};


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_content = read_to_string(file_name)?;
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut scroll: u16 = 0;

    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title(String::from(file_name))
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);

            let layout = Layout::default()
                            .margin(2)
                            .constraints(
                                [
                                Constraint::Percentage(100)
                                ].as_ref(),
                            )
                            .split(size);
    
            let paragraph = Paragraph::new(String::from(&file_content))
                            .style(Style::default())
                            .alignment(tui::layout::Alignment::Left)
                            .wrap(Wrap { trim: true })
                            .scroll((scroll, 0));

            f.render_widget(paragraph, layout[0]);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
            if key == Key::Down || key == Key::Char('j') {
                scroll += 1;
            }
            if key == Key::Up || key == Key::Char('k') {
                if scroll > 0 {
                    scroll -= 1;
                }
            }
        }
    };
    terminal.clear()?;
    Ok(())
}
