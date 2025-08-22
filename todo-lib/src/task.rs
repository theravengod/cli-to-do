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