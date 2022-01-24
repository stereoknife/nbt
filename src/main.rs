use std::io;
use std::time::Duration;

use crossterm::{
    execute,
    event::{EnableMouseCapture, DisableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}
};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use inputs::InputEvent;
use inputs::events::Events;
use inputs::key::Key;

mod consts;
mod region;
mod nbt;
mod inputs;

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        // Render
        term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30)
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                 .title("Block")
                 .borders(Borders::NONE);
            f.render_widget(block, chunks[0]);
            let block = Block::default()
                 .title("Block 2")
                 .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        match events.next().unwrap() {
            InputEvent::Input(Key::Char('q')) => break,
            InputEvent::Input(_) => (),
            InputEvent::Tick => (),
        };
    }

    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal::disable_raw_mode()?;
    Ok(())
}