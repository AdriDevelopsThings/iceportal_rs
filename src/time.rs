use chrono::{NaiveDateTime, NaiveDate};
use serde::{de, Deserializer, Deserialize};

pub fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where D: Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    println!("uwu {}", s);
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

pub fn naive_date_time_from_ms<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where D: Deserializer<'de>
{
    let i: Option<i64> = Option::deserialize(deserializer)?;
    Ok(match i {
        Some(s) => Some(
            NaiveDateTime::from_timestamp_millis(s).ok_or_else(|| de::Error::custom("Invalid ms timestamp"))?
        ),
        None => None
    })
}

pub fn delay_by_str<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where D: Deserializer<'de> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(match s {
        Some(i) => {
            if i.is_empty() {
                Some(0)
            } else {
                let mut raw_s: i32 = i[1..].parse().map_err(de::Error::custom)?;
                if i.starts_with('-') {
                    raw_s = -raw_s;
                }
                Some(raw_s)
            }
        }
        None => None,
    })
} 