use chrono::{Datelike, NaiveDate};

pub(crate) trait YearWeek {
    fn year_week(&self) -> String;
}

impl YearWeek for NaiveDate {
    fn year_week(&self) -> String {
        format!("{}W{}", self.year(), self.iso_week().week())
    }
}
