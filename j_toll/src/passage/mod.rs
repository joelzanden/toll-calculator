mod custom_serde_formats;
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
use custom_serde_formats::{my_date_format, my_time_format};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Passage {
    checkpoint_id: usize,
    #[serde(with = "my_date_format")]
    pub date: NaiveDate,
    #[serde(with = "my_time_format")]
    pub time: NaiveTime,
}

impl Passage {
    pub fn get_toll_fee(&self) -> u8 {
        if date_is_toll_free(&self.date) {
            0
        } else {
            match &self.time.hour() {
                5 => 8,
                6 => 18,
                7 => 28,
                8 => 18,
                9..=12 => 8,
                16 => 18,
                17..=19 => 28,
                _ => 0,
            }
        }
    }
}

fn date_is_toll_free(date: &NaiveDate) -> bool {
    date.weekday().number_from_monday() == 6 || date.weekday().number_from_monday() == 7
}
