pub mod app;

use std::{
    io,
    time::{Duration, Instant},
};

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    Frame, Terminal,
};

/// Rusty Xs and Os client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Activate nerdy developer stats
    #[arg(short, long, default_value_t = false)]
    developer_mode: bool,
}

pub fn run(cli: &mut Cli) -> Result<()> {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
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
                if let KeyCode::Esc | KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
            app.handle_key_event(&app_event);
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
}