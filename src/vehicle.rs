use uuid::Uuid;

impl Vehicle {
    pub fn new(name: String, speed: i32) -> Vehicle {
        Vehicle {
            id: Uuid::new_v4(),
            name,
            speed,
        }
    }
}
pub struct Vehicle {
    id: Uuid,
    pub name: String,
    pub speed: i32,
}
