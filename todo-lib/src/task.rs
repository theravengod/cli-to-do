use colored::Colorize;
use uuid::Uuid;

pub struct Task {
    pub id: Uuid,
    pub title: String,
    description: String,
    timestamp: std::time::SystemTime
}

impl Task {
    pub(crate) fn new(title: String, description: String) -> Task {
        Task {
            id: Uuid::new_v4(),
            title,
            description,
            timestamp: std::time::SystemTime::now()
        }
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
    fn pretty_print_with_count(&self, count: i32) -> String;
}

impl PrettyPrint for Task {
    fn pretty_print(&self) -> String {
        format!("{} {:?}", "Title: ".bright_blue(), self.title)
    }

    fn pretty_print_with_count(&self, count: i32) -> String {
        format!("[{}] {}", format!("{}", count).yellow(), self.pretty_print())
    }
}
