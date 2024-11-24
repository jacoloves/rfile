use std::{
    env,
    error::Error,
    fs::File,
    io::{stdout, Write},
    os::fd::AsRawFd,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use termios::{tcsetattr, Termios, ECHO, ICANON, OPOST, TCSANOW};

fn main() -> Result<(), Box<dyn Error>> {
    // get file path
    let file_path = get_file_path()?;

    // read csv file
    let (header_row, table) = read_csv_to_table(&file_path)?;

    // initialize terminal
    let originale_termios = init_terminal()?;

    // set page size
    let page_size = 30;

    // display table with paging
    let display_result = display_table(header_row, &table, page_size);

    // reset terminal
    reset_terminal(&originale_termios)?;

    // propagate error
    display_result?;

    Ok(())
}

fn get_file_path() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: csv_tool <file_path>".into());
    }

    Ok(args[1].clone())
}

fn read_csv_to_table(file_path: &str) -> Result<(Row, Table), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let headers = rdr.headers()?;
    let header_row = Row::new(headers.iter().map(|h| Cell::new(h)).collect());

    let mut table = Table::new();

    for res in rdr.records() {
        let record = res?;
        let record_row = Row::new(record.iter().map(|r| Cell::new(r)).collect());
        table.add_row(record_row);
    }

    Ok((header_row, table))
}

fn init_terminal() -> Result<Termios, Box<dyn Error>> {
    let stdin_fd = std::io::stdin().as_raw_fd();
    let original_termios = Termios::from_fd(stdin_fd)?;
    let mut raw_termios = original_termios;

    // enable raw mode
    terminal::enable_raw_mode()?;

    // enable OPOST and disable ICANON and ECHO
    raw_termios.c_lflag &= !(ICANON | ECHO);
    raw_termios.c_oflag |= OPOST;
    tcsetattr(stdin_fd, TCSANOW, &raw_termios)?;

    Ok(original_termios)
}

fn reset_terminal(original_termios: &Termios) -> Result<(), Box<dyn Error>> {
    let stdin_fd = std::io::stdin().as_raw_fd();
    tcsetattr(stdin_fd, TCSANOW, original_termios)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn display_table(header_row: Row, table: &Table, page_size: usize) -> Result<(), Box<dyn Error>> {
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
