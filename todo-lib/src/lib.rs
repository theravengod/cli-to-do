use colored::Colorize;
use console::Term;

pub fn run_mode(args: &Vec<String>) {
    if args.iter().len() > 0 {
        show_menu()
    } else {
        println!("With args")
    }
}

fn show_menu() {
    println!("..:: {} ::..\n", "To Do Manager".bright_white());
    println!("Possible actions:");
    println!("{}. Create a note", "1".yellow());
    println!("{}. Show a note", "2".yellow());
    println!("{}. Search for a note", "3".yellow());
    println!("{}. Show all notes titles", "4".yellow());
    println!("{}. Edit a note", "5".yellow());
    println!("{}. Delete a note", "6".yellow());
    println!("{}. Quit", "Q".red());

    print!("Choose your action: ");
    let term = Term::stdout();
    match (term.read_char()) {
        Ok(ch) => {
            let action = match (ch) {
                '1' => 1,
                '2' => 2,
                _ => -1
            };
            println!("Action: {}", action)
        }
        Err(_) => {}
    }
}