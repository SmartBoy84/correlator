use super::*;

impl From<Truck> for TruckModel {
    fn from(value: Truck) -> Self {
        Self {
            id: value.name,
            speed_map: value
                .timeframes
                .iter()
                .map(|a| {
                    a.velocity
                        .deltas
                        .windows(2)
                        .zip(a.acceleration.deltas.iter())
                        .map(|(v, a)| ((v[1].rate + v[0].rate) / 2.0, a.rate))
                })
                .flatten()
                .collect::<Vec<_>>(),
        }
    }
}

impl TruckModel {
    fn calculate_time(points: Vec<(f64, f64)>)
}