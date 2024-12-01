use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;

pub fn read_csv_to_table(file_path: &str) -> Result<Table, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut table = Table::new();

    // add header row
    let headers = rdr.headers()?;
    let header_row = Row::new(headers.iter().map(|h| Cell::new(h)).collect());
    table.add_row(header_row);

    // add data rows
    for result in rdr.records() {
        let record = result?;
        let row = Row::new(record.iter().map(|r| Cell::new(r)).collect());
        table.add_row(row);
    }

    Ok(table)
}
