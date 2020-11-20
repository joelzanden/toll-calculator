mod cli;
mod passage;
mod vehicle;

// use chrono::NaiveDate;
use vehicle::{Vehicle, VehicleTrait};

fn main() {
    let args = cli::parse();
    let vehicle = Vehicle::from_file(&args.input_file_path);

    if let true = &vehicle.is_toll_free() {
        println!("Toll free vehicle!");
    } else {
        println!("Kostnad för varje passage innan beräkningar:");
        // let mut total_fee = 0;
        // let mut current_date = NaiveDate::from_ymd(1983, 5, 30);
        for passage in &vehicle.passages {
            // if current_date == passage.date {
            //     println!("samma datum");
            // }
            println!(
                "{} {}: {} kr",
                passage.date,
                passage.time,
                passage.get_toll_fee()
            );
        }
    }
}
