use chrono::{DateTime, Utc};
use colored::Colorize;
use std::time::SystemTime;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Notebook {
    pub notes: Vec<Note>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Note {
    pub title: String,
    pub description: String,
    #[serde(with = "systemtime_serde")]
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

mod systemtime_serde {
    use std::time::UNIX_EPOCH;
    use serde::Serializer;
    use super::*;

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}
