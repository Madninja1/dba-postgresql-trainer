mod app;
mod event;
mod localization;
mod ui;

use std::{error::Error, fs, io};

use app::App;
use localization::UiLanguage;

use dba_trainer_application::TrainerService;
use dba_trainer_storage_sqlite::SqliteRepository;

use dba_trainer_content::load_builtin_bundles;
use ratatui::DefaultTerminal;

const DATA_DIRECTORY: &str = "data";
const DATABASE_PATH: &str = "data/dba_trainer.db";
const LANGUAGE_PATH: &str = "data/tui_language";

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(DATA_DIRECTORY)?;

    let mut repository = SqliteRepository::open(DATABASE_PATH)?;

    let bundles = load_builtin_bundles()?;

    for bundle in &bundles {
        repository.sync_bundle(bundle)?;
    }

    let service = TrainerService::new(repository);

    let app = App::new(service, load_language())?;

    ratatui::run(move |terminal| run(terminal, app))?;

    Ok(())
}

fn load_language() -> UiLanguage {
    fs::read_to_string(LANGUAGE_PATH)
        .ok()
        .and_then(|value| UiLanguage::from_code(value.trim()))
        .unwrap_or(UiLanguage::English)
}

fn save_language(language: UiLanguage) -> io::Result<()> {
    fs::write(LANGUAGE_PATH, language.code())
}

fn run(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if let Some(action) = event::read_action()? {
            let previous_language = app.language;
            app.handle_action(action);

            if app.language != previous_language {
                save_language(app.language)?;
            }
        }
    }

    Ok(())
}
