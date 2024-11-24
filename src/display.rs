use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use prettytable::{Row, Table};
use std::error::Error;
use std::io::{stdout, Write};

pub fn display_table(
    header_row: Row,
    table: &Table,
    page_size: usize,
) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let total_rows = table.len();
    let mut current_row = 0;

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), MoveTo(0, 0))?;
        let mut page_table = Table::new();

        page_table.add_row(header_row.clone());

        for i in 0..page_size {
            if current_row + i < total_rows {
                if let Some(row) = table.get_row(current_row + i) {
                    page_table.add_row(row.clone());
                }
            }
        }

        page_table.print(&mut stdout)?;
        stdout.flush()?;

        if current_row + page_size >= total_rows {
            println!("End of file");
        }

        // wait for key input
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter | KeyCode::Down | KeyCode::Char('j') => {
                        if current_row + page_size < total_rows {
                            current_row += page_size;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if current_row >= page_size {
                            current_row -= page_size;
                        } else {
                            current_row = 0;
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
