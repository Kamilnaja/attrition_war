use crate::vehicle_type::{self, VehicleType};
use uuid::Uuid; // Import the enum

impl Vehicle {
    pub fn new(name: String, speed: i32, vehicle_type: VehicleType) -> Vehicle {
        Vehicle {
            id: Uuid::new_v4(),
            name,
            speed,
            vehicle_type: VehicleType::Plane,
        }
    }
}
pub struct Vehicle {
    id: Uuid,
    pub name: String,
    pub speed: i32,
    pub vehicle_type: VehicleType,
}
