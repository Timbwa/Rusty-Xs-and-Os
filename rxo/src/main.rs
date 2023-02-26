use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    ops::Not,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

type Coordinate = (usize, usize);

struct App {
    active_cell_coordinate: Coordinate,
}

impl App {
    fn new() -> App {
        App {
            active_cell_coordinate: (0, 0),
        }
    }

    pub fn up(&mut self) {
        let (r, c) = self.active_cell_coordinate;
        if let Some(active_row) = r.checked_sub(1) {
            self.active_cell_coordinate = (active_row, c);
        }
    }
    pub fn down(&mut self) {
        let (r, c) = self.active_cell_coordinate;
        self.active_cell_coordinate = (r + usize::from(r < 3 - 1), c);
    }

    pub fn left(&mut self) {
        let (r, c) = self.active_cell_coordinate;
        if let Some(active_cell) = c.checked_sub(1) {
            self.active_cell_coordinate = (r, active_cell);
        }
    }
    pub fn right(&mut self) {
        let (r, c) = self.active_cell_coordinate;
        self.active_cell_coordinate = (r, c + usize::from(c < 3 - 1));
    }
}

struct GridCell<'a> {
    coordinate: Coordinate,
    app: &'a App,
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

    fn is_active(&self) -> bool {
        self.app.active_cell_coordinate == self.coordinate
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);
    // restore terminal

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(200);
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let app_event = event::read()?;
            if let Event::Key(key) = app_event {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                    // support vim motions
                    KeyCode::Down | KeyCode::Char('j') => app.down(),
                    KeyCode::Up | KeyCode::Char('k') => app.up(),
                    KeyCode::Right | KeyCode::Char('l') => app.right(),
                    KeyCode::Left | KeyCode::Char('h') => app.left(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Surrounding Block
    let block = Block::default()
        .title(" Rusty Xs and Os ")
        .borders(Borders::ALL)
        .border_type(tui::widgets::BorderType::Rounded)
        .title_alignment(tui::layout::Alignment::Center);
    f.render_widget(block, size);

    let cell_constraints = [
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
    ];

    let row_rects = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(1)
        .constraints(cell_constraints)
        .split(f.size());

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

            f.render_widget(cell_text, col_rect);
        }
    }
}
