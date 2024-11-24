use std::{env, error::Error};

pub fn get_file_path() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: csv_tool <file_path>".into());
    }

    Ok(args[1].clone())
}
