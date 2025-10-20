use std::{fs, io};
use std::path::Path;
use std::process::exit;
use colored::Colorize;
use crate::note::{Note, Notebook};

pub(crate) fn load_data(file_path: &str) -> Result<Vec<Note>, String> {
    let data_file_path = Path::new(file_path);

    match fs::read_to_string(data_file_path) {
        Ok(data) => {
            match serde_json::from_str::<Notebook>(data.as_str()) {
                Ok(parsed_data) => {
                    Ok(parsed_data.notes)
                },
                Err(_) => Err("Could not parse the previous stored notes.".to_string())
            }
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(Vec::new())
        },
        Err(_) => Err("Could not read previously stored notes.".to_string())
    }
}

pub(crate) fn save_data(file_path: &str, notes: Vec<Note>) {
    let output = serde_json::to_string(&Notebook{ notes } );
    if output.is_ok() {
        fs::write(file_path, output.unwrap()).expect("Error saving");
        println!("Data saved");
        exit(0)
    } else {
        eprintln!("{}", "Could not prepare data to save !".red());
        exit(1)
    }
}