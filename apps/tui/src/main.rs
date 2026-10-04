mod app;
mod event;
mod ui;

use std::io;

use app::App;

use ratatui::DefaultTerminal;

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if let Some(action) = event::read_action()? {
            app.handle_action(action);
        }
    }

    Ok(())
}
