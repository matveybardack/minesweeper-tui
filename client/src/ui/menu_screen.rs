use crate::config::Config;
use crate::error::AppError;
use crossterm::event;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<(), AppError> {
        const GREETING: &str = "Hello, world!";
        loop {
            terminal.draw(|frame| frame.render_widget(format!("{GREETING}"), frame.area()))?;
            if matches!(event::read()?, event::Event::Key(_)) {
                break Ok(());
            }
        }
    }
}
