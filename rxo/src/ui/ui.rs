use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::app::{App, AppState};

pub struct Ui;

impl Ui {
    pub fn render_ui<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>) {
        let size = frame.size();

        // Surrounding Block
        let block = Block::default()
            .title(" Rusty Xs and Os ")
            .borders(Borders::ALL)
            .border_type(tui::widgets::BorderType::Rounded)
            .title_alignment(tui::layout::Alignment::Center);
        frame.render_widget(block, size);

        match app.app_state {
            AppState::InitialMenu => self.render_menu(app, frame),
            AppState::RunningGame(_) => self.render_game(app, frame),
        }
    }

    fn render_menu<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(frame.size());

        let selected_style = Style::default()
            // .bg(Color::Rgb(153, 20, 54)) Purple
            .add_modifier(Modifier::REVERSED);
        let rows = app.menu_items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let t = Table::new(rows)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(selected_style)
            .highlight_symbol("* ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        frame.render_stateful_widget(t, rects[0], &mut app.menu_state);
    }

    fn render_game<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>) {}
}
