use std::{env, error::Error, fs::File};

use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;

    read_and_print_csv(file_path)?;

    Ok(())
}

fn read_and_print_csv(file_path: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut table = Table::new();

    let headers = rdr.headers()?;
    let header_row = Row::new(headers.iter().map(|h| Cell::new(h)).collect());
    table.add_row(header_row);

    for res in rdr.records() {
        let record = res?;
        let record_row = Row::new(record.iter().map(|r| Cell::new(r)).collect());
        table.add_row(record_row);
    }

    table.printstd();

    Ok(())
}

fn get_file_path() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: csv_tool <file_path>".into());
    }

    Ok(args[1].clone())
}
