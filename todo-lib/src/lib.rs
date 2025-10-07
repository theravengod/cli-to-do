use crate::note::Note;
use colored::Colorize;
use console::Term;
use std::io::Write;

mod note;

pub fn run_main() {
    let term = Term::stdout();
    let all_notes: Vec<Note> = Vec::new();

    show_menu(&term);
}

fn show_menu(term: &Term) {
    let mut selected_note: Option<Note> = None;

    println!("..:: {} ::..\n", "To Do Manager".bright_magenta());
    show_options(selected_note.is_some());

    let mut should_exit = false;
    while !should_exit {
        print!("Choose your action: ");
        std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let action = term.read_char();
        match action {
            Ok(ch) => {
                if selected_note.is_none() {
                    match ch {
                        '1' => println!("Show all"),
                        '2' => println!("Open note"),
                        '3' => println!("Search"),
                        'a' | 'A' => println!("Create"),
                        'q' | 'Q' => should_exit = true,
                        _ => println!("\nInvalid option !")
                    }
                } else {
                    match ch {
                        '1' => println!("Show content"),
                        '2' => println!("Edit title"),
                        '3' => println!("Edit desc"),
                        '4' => println!("Delete note"),
                        'c' | 'C' => println!("Clear selection"),
                        'q' | 'Q' => should_exit = true,
                        _ => println!("\nInvalid option !")
                    }
                }
            },
            Err(_) => {
                eprintln!("Something broke !");
                should_exit = true;
            }
        }
    }
}

fn show_options(has_opened_note: bool) {
    if !has_opened_note {
        println!("{}) Show all notes titles", "1".yellow());
        println!("{}) Open a note to view or edit", "2".yellow());
        println!("{}) Search for a note by title", "3".yellow());
        println!("{}) Add a new note", "A".bright_green());
    } else {
        println!("{}) Display the current note", "1".yellow());
        println!("{}) Edit the tile", "2".yellow());
        println!("{}) Edit the description", "3".yellow());
        println!("{}) Delete the current note", "4".yellow());
        println!("{}) Close the current note", "C".bright_blue());
    }
    println!("{}) Show possible actions", "H".cyan());
    println!("{}) Quit", "Q".red());
}