use jiff::civil::Date;

use crate::duration::{Duration, parse_duration};

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

    pub fn parse(input: &str) -> Result<Entry, String> {
        let (main, note) = match input.split_once(" | ") {
            Some((m, n)) => (m.trim(), Some(n.trim().to_string())),
            None => (input, None),
        };

        let (date_str, duration_str, tag_str) = split_twice(main).ok_or("invalid format")?;

        let date = date_str
            .parse::<Date>()
            .map_err(|_| format!("invalid date '{date_str}', expected YYYY-MM-DD"))?;

        let duration = parse_duration(duration_str)
            .map_err(|_| format!("invalid duration '{duration_str}', expected XhYm / Xh / Ym"))?;

        let tags: Vec<String> = tag_str.split(' ').map(str::to_string).collect();
        if tags.is_empty() {
            return Err("at least one tag is required".to_string());
        }

        Ok(Entry {
            date,
            duration,
            tags,
            note,
        })
    }
}

fn split_twice(input: &str) -> Option<(&str, &str, &str)> {
    let mut parts = input.splitn(3, " ");
    Some((parts.next()?, parts.next()?, parts.next()?))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entry_full() {
        let e = Entry::parse("2026-04-03 3h30m work/acme/meeting wfh | Kickoff").unwrap();
        assert_eq!(e.date, "2026-04-03".parse::<Date>().unwrap());
        assert_eq!(e.duration.minutes, 210);
        assert_eq!(e.tags, vec!["work/acme/meeting", "wfh"]);
        assert_eq!(e.note.as_deref(), Some("Kickoff"));
    }

    #[test]
    fn parse_entry_no_note() {
        let e = Entry::parse("2026-04-03 7h work/internal office").unwrap();
        assert!(e.note.is_none());
        assert_eq!(e.tags, vec!["work/internal", "office"]);
    }

    #[test]
    fn parse_entry_missing_tag() {
        assert!(Entry::parse("2026-04-03 1h").is_err());
    }

    #[test]
    fn parse_and_display() {
        let cases = vec![
            "2026-04-09 3h20m work/serious | some very serious stuff",
            "2026-04-03 3h30m work/acme/meeting wfh | Kickoff",
            "2026-04-03 7h work/internal office",
        ];

        for entry_str in cases {
            let entry = Entry::parse(entry_str).unwrap();
            assert_eq!(entry_str, entry.to_string());
        }
    }
}
