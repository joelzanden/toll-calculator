use super::passage::Passage;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Vehicle {
    uid: String,
    pub vehicle_type: VehicleType,
    pub passages: Vec<Passage>,
}

// #[derive(Deserialize, Debug)]
// pub struct Passage {
//     checkpoint_id: usize,
//     #[serde(with = "my_date_format")]
//     date: NaiveDate,
//     #[serde(with = "my_time_format")]
//     time: NaiveTime,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VehicleType {
    Motorbike,
    Tractor,
    Emergency,
    Diplomat,
    Foreign,
    Military,
    Car,
    Submarine,
    Truck,
}

impl Vehicle {
    pub fn from_file(path: &PathBuf) -> Vehicle {
        let content = std::fs::read_to_string(&path).expect("Couldn't read from file");
        serde_json::from_str(&content).unwrap()
    }
    // pub fn is_toll_free(&self) -> bool {
    //     match self.vehicle_type {
    //         VehicleType::Car | VehicleType::Truck => false,
    //         _ => true,
    //     }
    // }
}

// mod my_date_format {
//     use chrono::NaiveDate;
//     use serde::{self, Deserialize, Deserializer};

//     const FORMAT: &str = "%Y-%m-%d";

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
//     }
// }
// mod my_time_format {
//     use chrono::NaiveTime;
//     use serde::{self, Deserialize, Deserializer};

//     const FORMAT: &str = "%H:%M";

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
//     }
// }
