use chrono::{DateTime, Duration, Utc};

pub fn after(time: DateTime<Utc>) -> DateTime<Utc> {
    time + Duration::seconds(1_000_000_000)
}
