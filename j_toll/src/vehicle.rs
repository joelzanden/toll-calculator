use super::passage::Passage;
use serde::Deserialize;
use std::path::PathBuf;

pub trait VehicleTrait {
    fn from_file(path: &PathBuf) -> Vehicle;
    fn is_toll_free(&self) -> bool;
}

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

impl VehicleTrait for Vehicle {
    fn from_file(path: &PathBuf) -> Vehicle {
        let content = std::fs::read_to_string(&path).expect("Couldn't read from file");
        serde_json::from_str(&content).unwrap()
    }
    fn is_toll_free(&self) -> bool {
        !matches!(self.vehicle_type, VehicleType::Car | VehicleType::Truck)
    }
}
