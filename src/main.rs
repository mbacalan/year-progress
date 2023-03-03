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
        let year_start = self.get_year_start(year);
        let year_end = self.get_year_end(year);

        year_end.unwrap() - year_start.unwrap()
    }

    fn get_elapsed_duration(&self, year: i32) -> Duration {
        let year_start = self.get_year_start(year);

        self.now.date_naive() - year_start.unwrap()
    }

    pub fn get(self) -> f64 {
        let year_duration = self.get_year_duration(self.now.year());
        let elapsed_duration = self.get_elapsed_duration(self.now.year());

        elapsed_duration.num_seconds() as f64 / year_duration.num_seconds() as f64
    }
}

fn main() {
    let year_progress = YearProgress::new(Utc::now());

    println!("Year Progress: {:.2}%", year_progress.get() * 100.0);
}
