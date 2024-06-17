use crate::Truck;
pub mod truck;

struct TruckModel {
    id: String,
    speed_map: Vec<(f64, f64)>, // acceleration, velocity
}
