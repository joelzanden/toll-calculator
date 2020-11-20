use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Passage {
    checkpoint_id: usize,
    #[serde(with = "my_date_format")]
    date: NaiveDate,
    #[serde(with = "my_time_format")]
    time: NaiveTime,
}

mod my_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
mod my_time_format {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%H:%M";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
