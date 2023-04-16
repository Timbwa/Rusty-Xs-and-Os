use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::TableState;

pub type Coordinate = (usize, usize);

pub enum AppState {
    InitialMenu,
    RunningGame(Coordinate),
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

    pub fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match self.app_state {
            AppState::InitialMenu => match key_event.code {
                KeyCode::Down => self.select_next_menu_item(),
                KeyCode::Up => self.select_previous_menu_item(),
                KeyCode::Char('t') => self.toggle_app_state(),
                KeyCode::Enter => self.select_menu_item(),
                _ => {}
            },
            AppState::RunningGame(_active_coordinate) => match key_event.code {
                KeyCode::Char('j') => print!("J-KEY-DOWN"),
                KeyCode::Char('k') => print!("K-KEY-UP"),
                KeyCode::Char('t') => self.toggle_app_state(),
                _ => {}
            },
        }
    }

    fn select_menu_item(&mut self) {
        if let Some(index) = self.menu_state.selected() {
            match index {
                0 => {} // TODO: New Game
                1 => exit(0),
                _ => {}
            }
        }
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
