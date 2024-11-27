use std::error::Error;
use std::fs::File;
use crossterm::terminal;
use serde_json::Value;
use prettytable::{Cell, Row, Table};
use textwrap::wrap;

pub fn read_json_to_table(file_path: &str) -> Result<(Row, Table), Box<dyn Error>> {
    // oepn json file
    let file = File::open(file_path)?;

    // parse json data
    let json_data: Value = serde_json::from_reader(file)?;
    
    // get terminal width
    let(term_width, _) = terminal::size()?;
    let max_cell_width = (term_width as usize).saturating_sub(4);

    // function to wrap text
    fn wrap_text(text: &str, max_width: usize) -> String {
        let wrapped_lines = wrap(text, max_width);
        wrapped_lines.join("\n")
    }

    let mut table = Table::new();

    // check if json data is an array
    if let Some(array) = json_data.as_array() {
        // create headers
        let mut headers = Vec::new();
        if let Some(first_item) = array.first() {
            if let Some(obj) = first_item.as_object() {
                for key in obj.keys() {
                    headers.push(key.clone());
                }
            } else {
                return Err("JSON array does not contain objects".into());
            }
        } else {
            return Err("JSON array is empty".into());
        }

        // create header cell
        let header_cells = headers.iter().map(|h| {
            let wrapped_text = wrap_text(h, max_cell_width);
            let cell = Cell::new(&wrapped_text);
            cell.clone().style_spec("FW");
            cell
        }).collect::<Vec<Cell>>();
        let headers_row = Row::new(header_cells);

        // create rows
        for item in array {
            if let Some(obj) = item.as_object() {
                let mut cells = Vec::new();
                for key in &headers {
                    let value = obj.get(key).unwrap_or(&Value::Null);
                    let cell_value = match value {
                        Value::Null => "".to_string(),
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => "".to_string(),
                    };
                    let wrapped_text = wrap_text(&cell_value, max_cell_width);
                    let cell = Cell::new(&wrapped_text);
                    cell.clone().style_spec("FW");
                    cells.push(cell);
                }
                let row = Row::new(cells);
                table.add_row(row);
            }
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

        Ok((headers_row, table))
    } else {
        Err("Json file does not contain an array".into())
    }
}