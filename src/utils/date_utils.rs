use chrono::{NaiveDate, Duration, Datelike, Weekday};

// Format date as "ВС.DD.MM.YYYY"
pub fn format_date_russian(date: &NaiveDate) -> String {
    let weekday_prefix = match date.weekday() {
        Weekday::Mon => "ПН",
        Weekday::Tue => "ВТ",
        Weekday::Wed => "СР",
        Weekday::Thu => "ЧТ",
        Weekday::Fri => "ПТ",
        Weekday::Sat => "СБ",
        Weekday::Sun => "ВС",
    };
    
    format!("{}.{:02}.{:02}.{}", weekday_prefix, date.day(), date.month(), date.year())
}

// Get the start of the week (Monday) for a given date
pub fn start_of_week(date: &NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    *date - Duration::days(weekday as i64)
}

// Get the end of the week (Sunday) for a given date
pub fn end_of_week(date: &NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    *date + Duration::days(6 - weekday as i64)
}
