mod cli;
mod passage;
mod vehicle;

use vehicle::Vehicle;

fn main() {
    let args = cli::parse();
    let vehicle = Vehicle::from_file(&args.input_file_path);
    println!("vehicle: {:#?}", vehicle);
}
