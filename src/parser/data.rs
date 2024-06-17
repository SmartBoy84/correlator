use std::ops::Range;

use super::*;

impl Truck {
    pub fn push_raw_frame(&mut self, data: Vec<Vec<String>>) -> Result<&Self, TruckError> {
        self.timeframes
            .push(TimeFrame::from_raw(data).map_err(|e| TruckError::BadTimeframe(e))?);
        Ok(self)
    }

    pub fn create_truck(name: &str) -> Self {
        Self {
            name: name.to_string(),
            timeframes: vec![],
        }
    }
}

impl TimeFrame {
    fn from_raw(mut data: Vec<Vec<String>>) -> Result<Self, TimeFrameError> {
        let mut points = vec![];
        for point in data.drain(..) {
            points.push(
                DataPoint::create_point(&point)
                    .map_err(|e| TimeFrameError::BadDataPoint(point.join(", "), e))?,
            )
        }
        points.sort_by(|a, b| a.time.cmp(&b.time));
        Ok(Self { points })
    }

    pub fn get_range(&self) -> ((f64, f64), (f64, f64)) {
        let mut max_y = f64::MIN;
        let mut min_y = f64::MAX;

        let mut max_x = f64::MIN;
        let mut min_x = f64::MAX;

        for DataPoint { x, y, .. } in &self.points {
            if *x > max_x {
                max_x = *x;
            } else if *x < min_x {
                min_x = *x;
            }

            if *y > max_y {
                max_y = *y;
            } else if *y < min_y {
                min_y = *y;
            }
        }
        ((min_x,max_x), (min_y,max_y))
    }
}

impl DataPoint {
    fn create_point(data: &Vec<String>) -> Result<Self, DataPointErr> {
        if data.len() != 3 {
            return Err(DataPointErr::InsufficientEntries);
        }
        Ok(Self {
            time: NaiveDateTime::parse_from_str(&data[0], TIME_FORMAT)?,
            x: data[1].parse::<f64>()?,
            y: data[1].parse::<f64>()?,
        })
    }
}