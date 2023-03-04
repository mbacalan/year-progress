use chrono::{Datelike, DateTime, Duration, Utc, NaiveDate};

struct YearProgress {
    now: DateTime<Utc>,
}

impl YearProgress {
    pub fn new(now: DateTime<Utc>) -> YearProgress {
        YearProgress { now }
    }

    fn get_year_start(&self, year: i32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, 1, 1)
    }

    fn get_year_end(&self, year: i32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, 12, 31)
    }

    fn get_year_duration(&self, year: i32) -> Duration {
        let year_start = self.get_year_start(year).unwrap();
        let year_end = self.get_year_end(year).unwrap();

        year_end - year_start
    }

    fn get_elapsed_duration(&self, year: i32) -> Duration {
        let year_start = self.get_year_start(year).unwrap();

        self.now.date_naive() - year_start
    }

    pub fn get(self) -> f64 {
        let year_duration = self.get_year_duration(self.now.year()).num_seconds();
        let elapsed_duration = self.get_elapsed_duration(self.now.year()).num_seconds();

        elapsed_duration as f64 / year_duration as f64
    }
}

fn main() {
    let year_progress = YearProgress::new(Utc::now()).get();

    println!("Year Progress: {:.2}%", year_progress * 100.0);
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use super::*;

    #[test]
    fn past_year_progress() {
        let year_progress = YearProgress::new(Utc.with_ymd_and_hms(2022, 12, 31, 0, 0, 0).unwrap()).get();
        assert_eq!(year_progress * 100.0, 100.00);
    }

    #[test]
    fn future_year_progress() {
        let year_progress = YearProgress::new(Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap()).get();
        assert_eq!(year_progress * 100.0, 0.00);
    }
}
