use crate::note::{Displayable, Note};
use colored::Colorize;
use console::Term;
use std::io::Write;
use std::time::SystemTime;

mod note;

pub fn run_main() {
    let term = Term::stdout();
    let mut notebook: Vec<Note> = Vec::new();

    show_menu(&term, &mut notebook);
}

fn show_menu(term: &Term, notebook: &mut Vec<Note>) {
    let mut selected_note: Option<SystemTime> = None;

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
                        '1' => show_notes(notebook),
                        '2' => { selected_note = open_note(term, notebook) },
                        '3' => println!("Search"),
                        'a' | 'A' => {
                            notebook.push(add_new_note(term))
                        },
                        'h' | 'H' => show_options(selected_note.is_some()),
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
                        'h' | 'H' => show_options(selected_note.is_some()),
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

fn add_new_note(term: &Term) -> Note {
    print_header("Add a new note");

    print!("{}", "Title: ".bright_blue());
    let title = term.read_line().unwrap();

    print!("{}", "Description: ".bright_blue());
    let description = term.read_line().unwrap();

    let note = Note::new(title, description);
    println!("{}", "Note created\n".bright_green());
    note
}

fn show_notes(notebook: &Vec<Note>) {
    print_header("Show all notes");

    let mut counter: u32 = 1;
    for item in notebook {
        item.display_with_counter(counter);
        counter += 1;
    }
}

fn open_note(term: &Term, notebook: &Vec<Note>) -> Option<SystemTime> {
    print_header("Open a note");
    let mut selection: Option<SystemTime> = None;

    println!("{}) Select the note by number", "1".yellow());
    println!("{}) Show all notes titles", "2".yellow());
    println!("{}) Search by title", "3".yellow());
    print!("Choose your action: ");
    std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    let action = term.read_char().unwrap();
    match action {
        '1' => {
            print!("Enter the note number: ");
            let search_text = term.read_line().unwrap();
            let selected_timestamp = selection_search(notebook, &search_text)?.first().unwrap().timestamp;
            selection = Some(notebook
                .iter()
                .find(|nn| selected_timestamp == nn.timestamp)
                ?.timestamp)
        },
        '2' => show_notes(notebook),
        '3' => { selection = search_by_title(term, notebook) },
        _ => eprintln!("No such option ! Nothing selected.")
    }

    selection
}

fn search_by_title(term: &Term, notebook: &Vec<Note>) -> Option<SystemTime> {
    print_header("Search by title");

    print!("Enter the title or part of it: ");
    std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    let search_criteria = term.read_line().unwrap();
    if search_criteria.is_empty() {
        eprintln!("Nothing provided.");
        None
    } else {
        let findings: Vec<Note> = notebook
            .iter()
            .filter(|item| item.title.contains(search_criteria.as_str()))
            .cloned()
            .collect();

        print_and_select_a_note(term, &findings)
    }
}

fn print_and_select_a_note(term: &Term, notes: &Vec<Note>) -> Option<SystemTime> {
    if !notes.is_empty() {
        show_notes(notes);
        print!("\nInput the number of the note you want to select (or input {} exit):", "X".bright_red());
        std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let selection = term.read_line().unwrap();
        match selection.as_str() {
            "X" => {
                None
            },
            _ => {
                let nr = selection.trim_ascii().parse::<usize>().unwrap();
                Some(notes.get(nr - 1).unwrap().timestamp)
            }
        }
    } else { None }
}

// Util methods
fn print_header(title: &str) {
    println!("\n{}{}{}", "[".yellow(), title.bright_magenta(), "]".yellow());
}

fn selection_search<'a>(notebook: &'a Vec<Note>, search_text: &str) -> Option<Vec<&'a Note>> {
    if search_text.is_empty() {
        None
    } else {
        if search_text.trim_ascii().chars().all(|c| c.is_numeric()) {
            let index = search_text.trim_ascii().parse::<usize>().unwrap();
            Some(vec!(notebook.get(index - 1).unwrap()))
        } else {
            let findings: Vec<&Note> = notebook.into_iter()
                .filter(|&item| item.title.contains(search_text))
                .collect();

            if findings.is_empty() {
                None
            } else {
                Some(findings)
            }
        }
    }
}