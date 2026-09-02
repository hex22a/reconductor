use std::fmt::Display;

pub enum ScanStatus {
    InProgress,
    Done,
}

impl ScanStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ScanStatus::InProgress => "in progress",
            ScanStatus::Done => "done",
        }
    }
}

impl Display for ScanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
