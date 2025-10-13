use chrono::{DateTime, Utc};
use colored::Colorize;
use std::time::SystemTime;

#[derive(Clone)]
pub struct Note {
    pub title: String,
    pub description: String,
    pub timestamp: SystemTime,
}

impl Note {
    pub(crate) fn new(title: String, description: String) -> Note {
        Note {
            title,
            description,
            timestamp: SystemTime::now(),
        }
    }
}

pub trait Displayable {
    fn display_in_list_with_counter(&self, counter: u32);
    fn show_entire(&self, num: Option<usize>);
}

impl Displayable for Note {
    fn display_in_list_with_counter(&self, counter: u32) {
        let formatter: DateTime<Utc> = self.timestamp.into();
        println!("{}. {} {}",
                 counter.to_string().bright_green(),
                 self.title.bright_white(),
                 format!("[{}]", formatter.format("%F %T")).cyan()
        )
    }
    fn show_entire(&self, num: Option<usize>) {
        if num.is_some() {
            print!("[{}] ", num.unwrap().to_string().bright_green());
        }
        println!("{}", self.title.bright_blue());
        let formatter: DateTime<Utc> = self.timestamp.into();
        println!("Created at: {}\n", format!("{}", formatter.format("%F %T")).cyan());
        println!("{}", self.description)
    }
}