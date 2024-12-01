use crate::utils::get_terminal_width;
use crate::utils::wrap_text;
use prettytable::{Cell, Row, Table};
use serde_json::Value;
use std::error::Error;
use std::fs::File;

pub fn read_json_to_table(file_path: &str) -> Result<Table, Box<dyn Error>> {
    // calculate max width for wrapping text
    let max_cell_width = get_terminal_width().saturating_sub(4);

    // oepn json file and parse json data
    let file = File::open(file_path)?;
    let json_data: Value = serde_json::from_reader(file)?;

    let mut table = Table::new();

    if let Some(array) = json_data.as_array() {
        // create header row
        let headers = if let Some(first_item) = array.first() {
            if let Some(obj) = first_item.as_object() {
                obj.keys().cloned().collect::<Vec<_>>()
            } else {
                return Err("JSON array dones not contain objects".into());
            }
        } else {
            return Err("JSON array is empty".into());
        };

        // create header cells
        let header_cells = headers
            .iter()
            .map(|h| {
                let wrapped_text = wrap_text(h, max_cell_width);
                let cell = Cell::new(&wrapped_text);
                cell.clone().style_spec("FW");
                cell
            })
            .collect();
        // let header_row = Row::new(headers.iter().map(|h| Cell::new(h)).collect());
        let header_row = Row::new(header_cells);
        table.add_row(header_row);

        // add data rows
        for item in array {
            if let Some(obj) = item.as_object() {
                let cells = headers
                    .iter()
                    .map(|key| {
                        let value = obj.get(key).unwrap_or(&Value::Null);
                        let cell_text: String = match value {
                            Value::Null => "".to_string(),
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => "".to_string(),
                        };
                        let wrapped_text = wrap_text(&cell_text, max_cell_width);
                        let cell = Cell::new(&wrapped_text);
                        cell.clone().style_spec("FW");
                        cell
                    })
                    .collect();
                table.add_row(Row::new(cells));
            }
        }

        Ok(table)
    } else {
        Err("JSON data is not an array".into())
    }
}
