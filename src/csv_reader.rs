use crossterm::terminal;
use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;
use textwrap::wrap;

pub fn read_csv_to_table(file_path: &str) -> Result<(Row, Table), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // get terminal width
    let (term_width, _) = terminal::size()?;
    let max_cell_width = (term_width as usize).saturating_sub(4);

    let headers = rdr.headers()?;
    let header_cells = headers.iter().map(|h| {
        let wrapped_text = wrap_text(h, max_cell_width);
        let cell = Cell::new(&wrapped_text);
        cell.clone().style_spec("FW");
        cell
    }).collect::<Vec<Cell>>();
    let header_row = Row::new(header_cells);

    let mut table = Table::new();

    for res in rdr.records() {
        let record = res?;
        let cells = record.iter().map(|r| {
            let wrapped_text = wrap_text(r, max_cell_width);
            let cell = Cell::new(&wrapped_text);
            cell.clone().style_spec("FW");
            cell
        }).collect::<Vec<Cell>>();
        let record_row = Row::new(cells);
        table.add_row(record_row);
    }

    // setting table format
    use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};

    let format = FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(LinePosition::Top, LineSeparator::new('-', '+', '+', '+'))
        .separator(LinePosition::Title, LineSeparator::new('=', '+', '+', '+'))
        .separator(LinePosition::Bottom, LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build(); 
    table.set_format(format);

    Ok((header_row, table))
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let wrapped_lines = wrap(text, max_width);
    wrapped_lines.join("\n")
}