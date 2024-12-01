use std::{env, error::Error};

use textwrap::wrap;

pub fn get_file_path() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: csv_tool <file_path>".into());
    }

    Ok(args[1].clone())
}

pub fn wrap_text(text: &str, max_width: usize) -> String {
    let wrapped_lines = wrap(text, max_width);
    wrapped_lines.join("\n")
}

pub fn get_terminal_width() -> usize {
    if let Some(size) = termsize::get() {
        size.cols as usize
    } else {
        80
    }
}
