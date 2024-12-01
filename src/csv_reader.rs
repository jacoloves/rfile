use crate::utils::truncate_text;
use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;

pub fn read_csv_to_table(file_path: &str) -> Result<Table, Box<dyn Error>> {
    // setting max width for cells
    let max_cell_width = 30;

    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut table = Table::new();

    // setting table format
    // let format = FormatBuilder::new()
    //     .column_separator('|')
    //     .borders('|')
    //     .separator(LinePosition::Top, LineSeparator::new('-', '+', '+', '+'))
    //     .separator(LinePosition::Title, LineSeparator::new('=', '+', '+', '+'))
    //     .separator(LinePosition::Bottom, LineSeparator::new('-', '+', '+', '+'))
    //     .padding(1, 1)
    //     .build();
    // table.set_format(format);

    // add header row
    let headers = rdr.headers()?;
    let header_cells = headers
        .iter()
        .map(|h| {
            let turancated_text = truncate_text(h, max_cell_width);
            Cell::new(&turancated_text)
        })
        .collect();
    // let header_row = Row::new(headers.iter().map(|h| Cell::new(h)).collect());
    let header_row = Row::new(header_cells);
    table.add_row(header_row);

    // add data rows
    for result in rdr.records() {
        let record = result?;
        let cells = record
            .iter()
            .map(|r| {
                let truncated_text = truncate_text(r, max_cell_width);
                Cell::new(&truncated_text)
            })
            .collect();
        // let row = Row::new(record.iter().map(|r| Cell::new(r)).collect());
        let row = Row::new(cells);
        table.add_row(row);
    }

    Ok(table)
}
