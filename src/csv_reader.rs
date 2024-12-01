use crate::utils::get_terminal_width;
use crate::utils::wrap_text;
use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;

pub fn read_csv_to_table(file_path: &str) -> Result<Table, Box<dyn Error>> {
    // calculate max width for wrapping text
    let max_cell_width = get_terminal_width().saturating_sub(4);

    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut table = Table::new();

    // add header row
    let headers = rdr.headers()?;
    let header_cells = headers
        .iter()
        .map(|h| {
            let wrappded_text = wrap_text(h, max_cell_width);
            let cell = Cell::new(&wrappded_text);
            cell.clone().style_spec("FW");
            cell
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
                let wrappded_text = wrap_text(r, max_cell_width);
                let cell = Cell::new(&wrappded_text);
                cell.clone().style_spec("FW");
                cell
            })
            .collect();
        // let row = Row::new(record.iter().map(|r| Cell::new(r)).collect());
        let row = Row::new(cells);
        table.add_row(row);
    }

    Ok(table)
}
