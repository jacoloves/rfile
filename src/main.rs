mod csv_reader;
mod display;
mod terminal_utils;
mod utils;
mod json_reader;

use csv_reader::read_csv_to_table;
use display::display_table;
use std::{error::Error, path::Path};
use terminal_utils::{init_terminal, reset_terminal};
use utils::get_file_path;
use json_reader::read_json_to_table;


fn main() -> Result<(), Box<dyn Error>> {
    // get file path
    let file_path = get_file_path()?;

    // get file extension
    let extension = Path::new(&file_path).extension().and_then(|ext| ext.to_str()).unwrap_or("");

    // read function based on file extension
    let (header_row, table) = match extension {
        "csv" => read_csv_to_table(&file_path)?,
        "json" => read_json_to_table(&file_path)?,
        _ => return Err("Unsupported file extension. Please provide CSV or JSON file.".into()),
    };

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
