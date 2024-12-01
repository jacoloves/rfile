use prettytable::Table;
use std::error::Error;
use std::io;

pub fn display_table(table: &Table, page_size: usize) -> Result<(), Box<dyn Error>> {
    let total_rows = table.len();
    let mut current_row = 1;

    while current_row < total_rows {
        // create new table and add current page rows
        let mut page_table = Table::new();

        // add header row
        if let Some(header) = table.get_row(0) {
            page_table.add_row(header.clone());
        }

        // add data rows
        for i in 0..page_size {
            if current_row + i < total_rows {
                if let Some(row) = table.get_row(current_row + i) {
                    page_table.add_row(row.clone());
                }
            }
        }

        // output page table
        page_table.printstd();

        // check if there are more rows to display
        if current_row + page_size >= total_rows {
            break;
        }

        // check for user input
        println!("-- More -- (Press Enter to continue, q to quit)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        // update current row
        current_row += page_size;
    }

    Ok(())
}
