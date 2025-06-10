use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn naive_date_to_str<S>(v: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.format("%Y-%m-%d").to_string())
}

pub fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

pub fn naive_date_time_from_ms<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let i: Option<i64> = Option::deserialize(deserializer)?;
    Ok(match i {
        Some(s) => Some(
            DateTime::from_timestamp_millis(s)
                .ok_or_else(|| de::Error::custom("Invalid ms timestamp"))?
                .naive_utc(),
        ),
        None => None,
    })
}

pub fn naive_date_time_to_str<S>(
    v: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(v) = v {
        serializer.serialize_str(&v.format("%Y-%m-%dT%H:%M:%S").to_string())
    } else {
        serializer.serialize_none()
    }
}

pub fn delay_by_str<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
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
