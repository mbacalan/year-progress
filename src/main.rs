use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

struct YearProgress {
    datetime: DateTime<Utc>,
    year: i32,
}

impl YearProgress {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            datetime: now,
            year: now.year(),
        }
    }

    fn year_start(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 1, 1)
    }

    fn year_end(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 12, 31)
    }

    fn year_duration(&self) -> Option<Duration> {
        self.year_start()
            .zip(self.year_end())
            .map(|(start, end)| end - start)
    }

    fn elapsed_duration(&self) -> Option<Duration> {
        self.year_start()
            .map(|start| self.datetime.date_naive() - start)
    }

    pub fn progress(&self) -> Option<f64> {
        let year_duration = self.year_duration()?.num_seconds() as f64;
        let elapsed_duration = self.elapsed_duration()?.num_seconds() as f64;

        Some(elapsed_duration / year_duration)
    }
}

fn main() {
    if let Some(progress) = YearProgress::new(Utc::now()).progress() {
        println!("Year Progress: {:.2}%", progress * 100.0);
    } else {
        println!("Something went wrong!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{LocalResult, TimeZone};

    #[test]
    fn invalid_date() {
        assert!(matches!(
            Utc.with_ymd_and_hms(2022, 13, 32, 25, 60, 61),
            LocalResult::None
        ));
    }

    #[test]
    fn past_year_progress() {
        let year_progress = YearProgress::new(Utc.with_ymd_and_hms(2022, 12, 31, 0, 0, 0).unwrap())
            .progress()
            .expect("a valid progress value");

        assert_eq!(year_progress * 100.0, 100.00);
    }

    #[test]
    fn future_year_progress() {
        let year_progress = YearProgress::new(Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap())
            .progress()
            .expect("a valid progress value");

        assert_eq!(year_progress * 100.0, 0.00);
    }
}
