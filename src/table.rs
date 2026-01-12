use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};
use std::io;

struct NetworkDetails {
    header:,
    data:Vec<u8>,
    
}

pub fn Tab() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let rows = vec![
                Row::new(vec!["Alice", "24", "Engineer"]),
                Row::new(vec!["Bob", "30", "Designer"]),
                Row::new(vec!["Charlie", "28", "Product"]),
            ];

            let table = Table::new(
                rows,
                [
                    Constraint::Length(10),
                    Constraint::Length(5),
                    Constraint::Length(10),
                ],
            )
            .header(Row::new(vec!["Name", "Age", "Role"]).style(Style::default().fg(Color::Yellow)))
            .block(Block::default().title("Users").borders(Borders::ALL))
            .column_spacing(2);

            let chunks = Layout::default()
                .constraints([Constraint::Min(0)])
                .split(size);

            f.render_widget(table, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
