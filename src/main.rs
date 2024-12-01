mod csv_reader;
mod display;
mod json_reader;
mod utils;

use csv_reader::read_csv_to_table;
use json_reader::read_json_to_table;
use std::{error::Error, path::Path};
use utils::get_file_path;

fn main() -> Result<(), Box<dyn Error>> {
    // get file path
    let file_path = get_file_path()?;

    // get file extension
    let extension = Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    // read function based on file extension
    let table = match extension {
        "csv" => read_csv_to_table(&file_path)?,
        "json" => read_json_to_table(&file_path)?,
        _ => return Err("Unsupported file extension. Please provide CSV or JSON file.".into()),
    };

    // display table
    table.printstd();

    Ok(())
}
