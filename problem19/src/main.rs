use chrono::offset::TimeZone;
use chrono::Datelike;

fn main() {
    println!(
        "{}",
        ByDuration::new(
            chrono::Utc.ymd(1901, 1, 1),
            chrono::Utc.ymd(2000, 12, 31),
            chrono::Duration::days(1),
        )
        .filter(|date| date.weekday() == chrono::Weekday::Sun)
        .filter(|date| date.day() == 1)
        .count()
    );
}

struct ByDuration<Tz: TimeZone> {
    from: chrono::Date<Tz>,
    to: chrono::Date<Tz>,
    duration: chrono::Duration,
}

impl<Tz: TimeZone> ByDuration<Tz> {
    fn new(from: chrono::Date<Tz>, to: chrono::Date<Tz>, duration: chrono::Duration) -> ByDuration<Tz> {
        ByDuration {
            from,
            to,
            duration,
        }
    }
}

impl<Tz: TimeZone> Iterator for ByDuration<Tz> {
    type Item = chrono::Date<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_from = self.from.clone();
        let next = std::mem::replace(
            &mut self.from,
            old_from + self.duration
        );

        if next > self.to {
            None
        } else {
            Some(next)
        }
    }
}
