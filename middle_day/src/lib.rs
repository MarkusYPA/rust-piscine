use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if is_leap_year(year) {
        return None;
    }

    // Middle day: for 365 days, day 183
    let mid_day = NaiveDate::from_yo_opt(year as i32, 183).unwrap();
    Some(mid_day.weekday())
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}


#[cfg(test)]
mod tests;