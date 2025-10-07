use std::time::SystemTime;

pub struct Note {
    pub title: String,
    pub description: String,
    timestamp: std::time::SystemTime,
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