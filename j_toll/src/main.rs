mod cli;
mod passage;
mod vehicle;

use vehicle::Vehicle;

fn main() {
    let args = cli::parse();
    let vehicle = Vehicle::from_file(&args.input_file_path);
    println!("vehicle: {:#?}", vehicle);

    if let true = &vehicle.is_toll_free() {
        println!("Toll free vehicle!");
    } else {
        println!("Time to pay!");
    }
}
