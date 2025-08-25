mod task;

use crate::task::PrettyPrint;
use colored::Colorize;
use console::Term;
use std::io::Write;
use task::Task;

pub fn run_mode(args: &Vec<String>) {
    // Some inits
    let term = Term::stdout();
    let mut task_store: Vec<Task> = Vec::new();

    // Logic
    if args.iter().len() > 0 {
        show_menu(&term, &mut task_store)
    } else {
        println!("With args")
    }
}

fn show_menu(term: &Term, store: &mut Vec<Task>) {
    println!("..:: {} ::..\n", "To Do Manager".bright_white());
    show_actions();

    let mut should_exit = false;
    while !should_exit {
        print!("Choose your action: ");
        std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let action = term.read_char();
        match (action) {
            Ok(ch) => {
                match (ch) {
                    '1' => create_new_task(term, store),
                    '2' => search_note_by_title(term, store),
                    '3' => show_all_tasks(term, store),
                    'h' | 'H' => show_actions(),
                    'q' | 'Q' => should_exit = true, // TODO: handle error
                    _ => println!("Invalid option")
                };
            }
            Err(_) => {}
        }
    }
}

fn show_actions() {
    println!("Possible actions:");
    println!("{}) Create a task", "1".yellow());
    println!("{}) Search for a task by title", "2".yellow());
    println!("{}) Show all tasks titles", "3".yellow());
    println!("{}) Edit a task", "4".yellow());
    println!("{}) Delete a task", "5".yellow());
    println!("{}) Show possible actions", "H".cyan());
    println!("{}) Quit", "Q".red());
}

fn create_new_task(term: &Term, store: &mut Vec<Task>) {
    print_header("Creating a new task");
    // Title
    print!("{}", "Title:".bright_blue());
    let title = term.read_line().unwrap();
    // Description
    print!("{}", "Description:".bright_blue());
    let desc = term.read_line().unwrap();

    let note = Task::new(title, desc);
    match (store.iter().find(|&item| item.id == note.id)) {
        None => {
            store.push(note);
            println!("{}", "Task created\n".bright_green());
        }
        Some(_) => {
            eprintln!("A task with the same ID already exists - Your new task was {} created", "NOT".red());
            std::process::exit(1)
        }
    }
}

fn search_note_by_title(term: &Term, store: &mut Vec<Task>) {
    print_header("Search by title");
    print!("{}", "Search for:".bright_blue());
    let search_criteria = term.read_line().unwrap();
    let findings: Vec<&Task> = store.iter()
        .filter(|&item| item.title.contains(search_criteria.as_str()))
        .collect();

    if findings.is_empty() {
        println!("No tasks matching that title found.")
    } else {
        let count = format!("{:?}", &findings.iter().count());
        println!("Found {} task(s):", count.bright_cyan());
        let mut counter = 1;
        for item in &findings {
            println!("[{}] {}: {}", counter.to_string().yellow(), "Title: ".bright_blue(),  item.title);
            counter += 1;
        }
    }
}

fn show_all_tasks(term: &Term, store: &mut Vec<Task>) {
    print_header("Show all tasks");
    let mut counter = 1;
    for item in store {
        println!("{}", item.pretty_print_with_count(counter));
        counter += 1;
    }
}

fn cls(term: &Term, lines: usize) {
    term.clear_last_lines(lines).unwrap()
}

fn print_header(title: &str) {
    println!("\n{}{}{}", "[".yellow(), title.bright_magenta(), "]".yellow());
}