pub(crate) trait YearWeek {
    fn year_week(&self) -> String;
}

impl YearWeek for time::Date {
    fn year_week(&self) -> String {
        format!("{}W{}", self.year(), self.iso_week())
    }
}
