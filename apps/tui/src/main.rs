mod app;
mod event;
mod ui;

use std::{error::Error, fs, io};

use app::App;

use dba_trainer_application::TrainerService;
use dba_trainer_storage_sqlite::SqliteRepository;

use dba_trainer_content::load_builtin_bundles;
use ratatui::DefaultTerminal;

const DATA_DIRECTORY: &str = "data";
const DATABASE_PATH: &str = "data/dba_trainer.db";

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(DATA_DIRECTORY)?;

    let mut repository = SqliteRepository::open(DATABASE_PATH)?;

    let bundles = load_builtin_bundles()?;

    for bundle in &bundles {
        repository.sync_bundle(bundle)?;
    }

    let service = TrainerService::new(repository);

    let app = App::new(service)?;

    ratatui::run(move |terminal| run(terminal, app))?;

    Ok(())
}

fn run(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
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
