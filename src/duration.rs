#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Duration {
    pub minutes: u64,
}

impl Duration {
    pub const ZERO: Self = Self { minutes: 0 };

    pub fn from_parts(hours: u64, minutes: u64) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn hours(self) -> u64 {
        self.minutes / 60
    }
    pub fn mins(self) -> u64 {
        self.minutes % 60
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.hours(), self.mins()) {
            (0, m) => write!(f, "{m}m"),
            (h, 0) => write!(f, "{h}h"),
            (h, m) => write!(f, "{h}h{m}m"),
        }
    }
}

impl std::ops::Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            minutes: self.minutes + rhs.minutes,
        }
    }
}

impl std::ops::AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.minutes += rhs.minutes;
    }
}

impl std::iter::Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

/// Parse `XhYm`, `Xh`, or `Ym`. Returns `None` if the string doesn't match.
pub fn parse_duration(s: &str) -> Result<Duration, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }
    if !s.contains('h') && !s.contains('m') {
        return Err("no 'h' or 's' in input".to_string());
    }
    let mut hours: u64 = 0;
    let mut minutes: u64 = 0;
    let mut rest = s;

    if let Some(h_pos) = rest.find('h') {
        hours = rest[..h_pos]
            .parse()
            .map_err(|_| "invalid input before 'h'".to_string())?;
        rest = &rest[h_pos + 1..];
    }
    if let Some(m_pos) = rest.find('m') {
        minutes = rest[..m_pos]
            .parse()
            .map_err(|_| "invalid input before 'm'".to_string())?;
        rest = &rest[m_pos + 1..];
    }
    if !rest.is_empty() {
        return Err(format!("trailing characters '{rest}'"));
    }
    Ok(Duration::from_parts(hours, minutes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_display() {
        assert_eq!(Duration::from_parts(3, 30).to_string(), "3h30m");
        assert_eq!(Duration::from_parts(3, 0).to_string(), "3h");
        assert_eq!(Duration::from_parts(0, 45).to_string(), "45m");
        assert_eq!(Duration::from_parts(0, 0).to_string(), "0m");
        assert_eq!(Duration::from_parts(0, 90).to_string(), "1h30m");
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("3h30m"), Ok(Duration::from_parts(3, 30)));
        assert_eq!(parse_duration("3h"), Ok(Duration::from_parts(3, 0)));
        assert_eq!(parse_duration("45m"), Ok(Duration::from_parts(0, 45)));
        assert_eq!(parse_duration("0m"), Ok(Duration::from_parts(0, 0)));
        assert!(matches!(parse_duration("1.5h"), Err(_)));
        assert!(matches!(parse_duration("bad"), Err(_)));
        assert!(matches!(parse_duration(""), Err(_)));
    }
}
