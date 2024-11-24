use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;

pub fn read_csv_to_table(file_path: &str) -> Result<(Row, Table), Box<dyn Error>> {
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
