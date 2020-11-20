mod custom_serde_formats;

use chrono::{NaiveDate, NaiveTime};
use custom_serde_formats::{my_date_format, my_time_format};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Passage {
    checkpoint_id: usize,
    #[serde(with = "my_date_format")]
    date: NaiveDate,
    #[serde(with = "my_time_format")]
    time: NaiveTime,
}

impl Passage {
    pub fn get_toll_fee(&self) -> u8 {
        60
    }
}
