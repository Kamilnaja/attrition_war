use vehicle::Vehicle;
use vehicle_type::VehicleType;
mod vehicle;
pub mod vehicle_type;
fn main() {
    println!("Hello, world!");
    start_game();
}

fn start_game() {
    let vehicle_1 = Vehicle::new(String::from("Borsuk"), 20, VehicleType::Land);
    let vehicle_2 = Vehicle::new(String::from("Panther"), 35, VehicleType::Land);

    println!("name: {} speed: {}", vehicle_1.name, vehicle_1.speed);
    println!("name: {} speed: {}", vehicle_2.name, vehicle_2.speed);
}
