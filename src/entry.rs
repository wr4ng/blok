use jiff::civil::Date;

use crate::duration::Duration;

#[derive(Debug, Clone)]
pub struct Entry {
    pub date: Date,
    pub duration: Duration,
    pub tags: Vec<String>,
    pub note: Option<String>,
}
