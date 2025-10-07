mod old_task;

use crate::old_task::PrettyPrint;
use colored::Colorize;
use console::Term;
use old_task::Task;
use std::io::Write;
use uuid::Uuid;

pub fn run_mode(args: &Vec<String>) {
    // Some inits
    let term = Term::stdout();
    let mut task_store: Vec<Task> = Vec::new();
    let mut active_task_id: Option<Uuid> = None;

    // Logic
    if args.iter().len() > 0 {
        show_menu(&term, &mut task_store, active_task_id)
    } else {
        println!("With args")
    }
}

fn show_menu(term: &Term, store: &mut Vec<Task>, mut active_task_id: Option<Uuid>) {
    println!("..:: {} ::..\n", "To Do Manager".bright_white());
    show_actions(active_task_id.is_some());

    let mut should_exit = false;
    while !should_exit {
        print!("Choose your action: ");
        std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let action = term.read_char();
        match action {
            Ok(ch) => {
                if active_task_id.is_none() {
                    match ch {
                        '1' => create_new_task(term, store),
                        '2' => search_note_by_title(term, store),
                        '3' => show_all_tasks(term, store),
                        '4' => {
                            active_task_id = select_a_task(term, store);
                        }
                        'h' | 'H' => show_actions(active_task_id.is_some()),
                        'q' | 'Q' => should_exit = true, // TODO: handle error
                        _ => println!("Invalid option")
                    };
                } else {
                    match ch {
                        '1' => println!("Show task"),
                        '2' => println!("Edit task title"),
                        '3' => println!("Edit task description"),
                        '4' => println!("Delete task"),
                        '5' => println!("Clear task"),
                        'h' | 'H' => show_actions(active_task_id.is_some()),
                        'q' | 'Q' => should_exit = true, // TODO: handle error
                        _ => println!("Invalid option")
                    };
                }
            }
            Err(_) => {}
        }
    }
}

fn show_actions(has_active_task: bool) {
    println!("Possible actions:");
    if !has_active_task {
        println!("{}) Create a task", "1".yellow());
        println!("{}) Search for a task by title", "2".yellow());
        println!("{}) Show all tasks titles", "3".yellow());
        println!("{}) Select a task by number", "4".yellow());
    } else {
        println!("{}) Show all task info", "1".yellow());
        println!("{}) Edit the tile", "2".yellow());
        println!("{}) Edit the description", "3".yellow());
        println!("{}) Delete the current task", "4".yellow());
        println!("{}) Clear the selected current task", "5".yellow());
    }
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
            println!("[{}] {}: {}", counter.to_string().yellow(), "Title: ".bright_blue(), item.title);
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

fn select_a_task(term: &Term, store: &mut Vec<Task>) -> Option<Uuid> {
    print_header("Select a task");
    print!("{}", "Enter the task number or part of its title:".bright_blue());
    let searched_text = term.read_line().unwrap();
    if searched_text.is_empty() {
        println!("No tasks matching that number or title found.");
        return None;
    }
    let results = selection_search(term, store, searched_text.as_str())
        .expect("No tasks matching that number or title found.");

    if results.iter().len() == 1 {
        let position = store.iter().position(|t| t.id == results[0]).unwrap();
        let task = store.iter().nth(position).unwrap();
        println!("Select task : {} ? [y/n]", task.pretty_print_with_count(position as i32));
        Some(task.id)
    } else {
        let findings: Vec<&Task> = store.iter().filter(|&item| results.contains(&item.id)).collect();
        let count = format!("{:?}", findings.iter().count());
        println!("Found {} task(s):", count.bright_cyan());
        for item in &findings {
            let counter = store.iter().position(|t| t.id == item.id).unwrap() + 1;
            println!("[{}] {}: {}", counter.to_string().yellow(), "Title: ".bright_blue(), item.title);
        }
        println!("Select the task number:");
        Some(findings.first().unwrap().id)
    }
}

fn selection_search(term: &Term, store: &mut Vec<Task>, search_text: &str) -> Option<Vec<Uuid>> {
    if search_text.is_empty() {
        None
    } else {
        if search_text.trim_ascii().chars().all(|c| c.is_numeric()) {
            let index = search_text.trim_ascii().parse::<usize>().unwrap();
            Some(vec!(store.get(index - 1)?.id))
        } else {
            let findings: Vec<&Task> = store.iter()
                .filter(|&item| item.title.contains(search_text))
                .collect();

            if findings.is_empty() {
                None
            } else {
                Some(findings.iter().map(|item| item.id).collect())
            }
        }
    }
}

fn cls(term: &Term, lines: usize) {
    term.clear_last_lines(lines).unwrap()
}

fn print_header(title: &str) {
    println!("\n{}{}{}", "[".yellow(), title.bright_magenta(), "]".yellow());
}