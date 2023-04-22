use std::ops::Not;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, AppState, Coordinate};

struct GridCell<'a> {
    coordinate: Coordinate,
    app: &'a App<'a>,
}

impl<'a> GridCell<'a> {
    fn create_cell_block(&self) -> Block<'a> {
        Block::default()
            .borders(self.determine_borders())
            .border_type(tui::widgets::BorderType::Thick)
            .style(Style::default().bg(if self.is_active() {
                Color::Rgb(153, 20, 204)
            } else {
                Color::Reset
            }))
    }

    fn determine_borders(&self) -> Borders {
        match self.app.app_state {
            AppState::InitialMenu => panic!("Invalid State"),
            AppState::RunningGame(_) => {
                let mut borders = Borders::NONE;
                let (r, c) = self.coordinate;

                if r == 0 {
                    borders = borders.union(Borders::TOP).not();
                }

                if r == 2 {
                    borders = borders.union(Borders::BOTTOM).not();
                }

                if c == 0 {
                    borders = borders.difference(Borders::LEFT);
                }

                if c == 2 {
                    borders = borders.difference(Borders::RIGHT);
                }

                if c > 0 && c < 2 {
                    borders = borders.union(Borders::RIGHT);
                    borders = borders.union(Borders::LEFT);
                }

                borders
            }
        }
    }

    fn is_active(&self) -> bool {
        match self.app.app_state {
            AppState::InitialMenu => false,
            AppState::RunningGame(active_cell_coordinate) => {
                return active_cell_coordinate == self.coordinate;
            }
        }
    }
}

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

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(size);

        // header
        let header_block = Block::default().borders(Borders::NONE);
        frame.render_widget(header_block, chunks[0]);

        match app.app_state {
            AppState::InitialMenu => self.render_menu(app, frame, chunks[1]),
            AppState::RunningGame(_) => self.render_game(app, frame, chunks[1]),
        }

        // footer
        self.render_footer(app, frame, chunks[2]);
    }

    fn render_menu<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>, layout_area: Rect) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(layout_area);

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

    fn render_footer<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>, layout_area: Rect) {
        let row_chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(layout_area);

        let author_description = vec![Spans::from("Github: Timbwa/Rusty-Xs-and-Os")];

        let author_description = Paragraph::new(author_description)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        let quit_game_description = vec![Spans::from("x: Exit game")];

        let quit_game_description = Paragraph::new(quit_game_description)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        if let AppState::InitialMenu = app.app_state {
            frame.render_widget(author_description, row_chunks[0]);
        }
        if let AppState::RunningGame(_) = app.app_state {
            frame.render_widget(quit_game_description, row_chunks[0]);
        }

        let instructions = vec![Spans::from("←↑↓→/hjkl to navigate")];
        let instructions = Paragraph::new(instructions)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(instructions, row_chunks[1]);

        let select_instruction = vec![Spans::from("↩ to select")];
        let select_instruction = Paragraph::new(select_instruction)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(select_instruction, row_chunks[2]);
    }

    fn render_game<B: Backend>(&self, app: &mut App, frame: &mut Frame<B>, layout_area: Rect) {
        let cell_constraints = [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ];

        let row_rects = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(1)
            .constraints(cell_constraints)
            .split(layout_area);

        for (r, row_rect) in row_rects.into_iter().enumerate() {
            let col_rects = Layout::default()
                .direction(tui::layout::Direction::Horizontal)
                .horizontal_margin(0)
                .constraints(cell_constraints)
                .split(row_rect);

            for (c, col_rect) in col_rects.into_iter().enumerate() {
                let text = format!("({c:?},{r:?})");
                let grid_cell = GridCell {
                    coordinate: (r, c),
                    app,
                };

                let cell_text = Paragraph::new(text)
                    .block(grid_cell.create_cell_block())
                    .alignment(tui::layout::Alignment::Center);

                frame.render_widget(cell_text, col_rect);
            }
        }
    }
}
