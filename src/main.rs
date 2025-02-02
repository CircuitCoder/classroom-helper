#[macro_use]
extern crate clap;
extern crate config;

use crossterm::event::KeyCode;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::ExecutableCommand;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod configs;
mod events;
mod model;
mod view;
mod execute;

fn main() -> Result<(), io::Error> {
    let config = configs::Config::new();

    // setup term
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let events = events::Events::new();
    let mut model = model::Model::new(config);

    loop {
        terminal.draw(|mut f| {
            view::draw(&mut model, &mut f);
        })?;

        match events.next().unwrap() {
            events::Event::Input(key) => match key.code {
                KeyCode::Char('q') => break,
                _ => {
                    model.handle(key.code);
                }
            },
            _ => {}
        }

        model.tick();
    }

    Ok(())
}
