use vehicle::Vehicle;
mod vehicle;
fn main() {
    println!("Hello, world!");
    start_game();
}

fn start_game() {
    let vehicle_1 = Vehicle::new(String::from("Borsuk"), 20);

    println!("name: {} speed: {}", vehicle_1.name, vehicle_1.speed)
}
