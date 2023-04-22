use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::TableState;

pub type Coordinate = (usize, usize);

pub enum AppState {
    InitialMenu,
    RunningGame(Coordinate),
}

pub enum AppAction {
    Exit,
}

pub struct App<'a> {
    pub app_state: AppState,
    pub menu_state: TableState,
    pub menu_items: Vec<Vec<&'a str>>,
    pub is_developer_mode: bool,
}

impl<'a> App<'a> {
    pub fn new(is_developer_mode_activated: bool) -> App<'a> {
        App {
            app_state: AppState::InitialMenu,
            menu_state: TableState::default(),
            menu_items: vec![vec!["1. New Game"], vec!["2. Quit (q)"]],

            is_developer_mode: is_developer_mode_activated,
        }
    }

    pub fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<AppAction> {
        match self.app_state {
            AppState::InitialMenu => match key_event.code {
                KeyCode::Down => {
                    self.select_next_menu_item();
                    return None;
                }
                KeyCode::Up => {
                    self.select_previous_menu_item();
                    return None;
                }
                KeyCode::Char('t') => {
                    self.toggle_app_state();
                    return None;
                }
                KeyCode::Enter => self.select_menu_item(),
                _ => None,
            },
            AppState::RunningGame(_active_coordinate) => match key_event.code {
                KeyCode::Char('j') => {
                    print!("J-KEY-DOWN");
                    return None;
                }
                KeyCode::Char('k') => {
                    print!("K-KEY-UP");
                    return None;
                }
                KeyCode::Char('t') => {
                    self.toggle_app_state();
                    return None;
                }
                _ => None,
            },
        }
    }

    fn select_menu_item(&mut self) -> Option<AppAction> {
        if let Some(index) = self.menu_state.selected() {
            match index {
                0 => return None, // TODO: New Game
                1 => return Some(AppAction::Exit),
                _ => return None,
            }
        }
        None
    }

    fn select_next_menu_item(&mut self) {
        let i = match self.menu_state.selected() {
            None => 0,
            Some(i) => {
                if i >= self.menu_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
        };

        self.menu_state.select(Some(i));
    }

    fn select_previous_menu_item(&mut self) {
        let i = match self.menu_state.selected() {
            None => 0,
            Some(i) => {
                if i == 0 {
                    self.menu_items.len() - 1
                } else {
                    i - 1
                }
            }
        };

        self.menu_state.select(Some(i));
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
