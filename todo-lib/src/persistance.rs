use std::fs;
use std::process::exit;
use colored::Colorize;
use crate::note::{Note, Notebook};

fn load_data() {

}

pub(crate) fn save_data(notes: Vec<Note>) {
    let output = serde_json::to_string(&Notebook{ notes } );
    if output.is_ok() {
        fs::write("./storage.json", output.unwrap()).expect("Error saving");
        println!("Data saved");
        exit(0)
    } else {
        eprintln!("{}", "Could not prepare data to save !".red());
        exit(1)
    }
}