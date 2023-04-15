use crossterm::event::{Event, KeyCode};

pub type Coordinate = (usize, usize);

pub enum AppState {
    InitialMenu,
    RunningGame(Coordinate),
}

pub struct App {
    pub app_state: AppState,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::InitialMenu,
        }
    }

    pub fn handle_key_event(&mut self, event: &Event) {
        match self.app_state {
            AppState::InitialMenu => {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Down => print!("ARROW-KEY-DOWN"),
                        KeyCode::Up => print!("ARROW-KEY-UP"),
                        KeyCode::Char('t') => self.toggle_app_state(),
                        _ => print!("Initial Menu No-Op"),
                    }
                }
            }
            AppState::RunningGame(_active_coordinate) => {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Char('j') => print!("J-KEY-DOWN"),
                        KeyCode::Char('k') => print!("K-KEY-UP"),
                        KeyCode::Char('t') => self.toggle_app_state(),
                        _ => print!("Running Game No-Op"),
                    }
                }
            }
        }
    }

    fn toggle_app_state(&mut self) {
        match self.app_state {
            AppState::InitialMenu => {
                self.app_state = AppState::RunningGame((0, 0));
            }
            AppState::RunningGame(_active_coordinate) => {
                self.app_state = AppState::InitialMenu;
            }
        }
    }
}
