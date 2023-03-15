use chrono::{Datelike, DateTime, Duration, Utc, NaiveDate};

struct YearProgress {
    datetime: DateTime<Utc>,
    year: i32
}

impl YearProgress {
    pub fn new(now: DateTime<Utc>) -> YearProgress {
        YearProgress {
            datetime: now,
            year: now.year()
        }
    }

    fn get_year_start(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 1, 1)
    }

    fn get_year_end(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 12, 31)
    }

    fn get_year_duration(&self) -> Option<Duration> {
        let year_start = self.get_year_start()?;
        let year_end = self.get_year_end()?;

        Some(year_end - year_start)
    }

    fn get_elapsed_duration(&self) -> Option<Duration> {
        let year_start = self.get_year_start()?;

        Some(self.datetime.date_naive() - year_start)
    }

    pub fn get(self) -> Option<f64> {
        let year_duration = self.get_year_duration()?.num_seconds();
        let elapsed_duration = self.get_elapsed_duration()?.num_seconds();

        Some(elapsed_duration as f64 / year_duration as f64)
    }
}

fn main() {
    match YearProgress::new(Utc::now()).get() {
        Some(x) => println!("Year Progress: {:.2}%", x * 100.0),
        None => println!("Something went wrong!")
    };
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, LocalResult};
    use super::*;

    #[test]
    fn invalid_date() {
        let invalid_date = match Utc.with_ymd_and_hms(2022, 13, 32, 25, 60, 61) {
            LocalResult::Single(x) => x,
            LocalResult::Ambiguous(x, _) => x,
            LocalResult::None => return ()
        };

        assert_eq!(invalid_date.month(), 1);
    }

    #[test]
    fn past_year_progress() {
        let year_progress = match YearProgress::new(Utc.with_ymd_and_hms(2022, 12, 31, 0, 0, 0).unwrap()).get() {
            Some(x) => x,
            None => 0.0
        };

        assert_eq!(year_progress * 100.0, 100.00);
    }

    #[test]
    fn future_year_progress() {
        let year_progress = match YearProgress::new(Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap()).get() {
            Some(x) => x,
            None => 0.0
        };

        assert_eq!(year_progress * 100.0, 0.00);
    }
}
