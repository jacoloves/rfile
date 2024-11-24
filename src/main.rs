mod csv_reader;
mod display;
mod terminal_utils;
mod utils;

use csv_reader::read_csv_to_table;
use display::display_table;
use std::error::Error;
use terminal_utils::{init_terminal, reset_terminal};
use utils::get_file_path;

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
