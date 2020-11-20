use super::passage::Passage;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Vehicle {
    uid: String,
    pub vehicle_type: VehicleType,
    pub passages: Vec<Passage>,
}

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
    pub fn is_toll_free(&self) -> bool {
        match self.vehicle_type {
            VehicleType::Car | VehicleType::Truck => false,
            _ => true,
        }
    }
}
