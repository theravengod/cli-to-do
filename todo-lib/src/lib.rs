mod task;

use colored::Colorize;
use console::Term;
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
    println!("Possible actions:");
    println!("{}. Create a task", "1".yellow());
    println!("{}. Search for a task by title", "2".yellow());
    println!("{}. Show all tasks titles", "3".yellow());
    println!("{}. Edit a task", "4".yellow());
    println!("{}. Delete a task", "5".yellow());
    println!("{}. Quit", "Q".red());

    let mut should_exit = false;
    while !should_exit {
        print!("Choose your action: ");
        let action = term.read_char();
        match (action) {
            Ok(ch) => {
                match (ch) {
                    '1' => create_new_task(&term, store),
                    '2' => search_note_by_title(term, store),
                    _ => { should_exit = true } // TODO: handle error
                };
            }
            Err(_) => {}
        }
    }
}

fn create_new_task(term: &Term, store: &mut Vec<Task>) {
    println!("{}", "[Creating a new task]".bright_white());
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
            println!("{}", "Task created".bright_green())
        }
        Some(_) => {
            eprintln!("A task with the same ID already exists - Your new task was {} created", "NOT".red());
            std::process::exit(1)
        }
    }
}

fn search_note_by_title(term: &Term, store: &mut Vec<Task>) {
    println!("{}", "[Search by title]".bright_white());
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