use jiff::civil::Date;

use crate::duration::Duration;

#[derive(Debug, Clone)]
pub struct Entry {
    pub date: Date,
    pub duration: Duration,
    pub tags: Vec<String>,
    pub note: Option<String>,
}

impl Entry {
    pub fn new(date: Date, duration: Duration, tags: Vec<String>, note: Option<String>) -> Self {
        Self {
            date,
            duration,
            tags,
            note,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date;
        let duration = self.duration;
        let tags = self.tags.join(" ");
        match &self.note {
            Some(note) => write!(f, "{date} {duration} {tags} | {note}"),
            None => write!(f, "{date} {duration} {tags}"),
        }
    }
}
