use std::any;

use plotters::{
    backend::{BitMapBackend, RGBPixel},
    chart::ChartBuilder,
    drawing::DrawingArea,
    element::Circle,
    series::LineSeries,
    style::{Color, RED, TRANSPARENT},
};

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
        let velocity = DeltaTimes::from_points(&points);
        let acceleration = DeltaTimes::from_self(&velocity);

        Ok(Self {
            points,
            velocity,
            acceleration,
        })
    }

    pub fn get_range(&self) -> ((f64, f64), (f64, f64)) {
        let mut max_y = f64::MIN;
        let mut min_y = f64::MAX;

        let mut max_x = f64::MIN;
        let mut min_x = f64::MAX;

        for DataPoint { x, y, .. } in &self.points {
            if *x > max_x {
                max_x = *x;
            }
            if *x < min_x {
                min_x = *x;
            }

            if *y > max_y {
                max_y = *y;
            }
            if *y < min_y {
                min_y = *y;
            }
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

impl DeltaTimes {
    fn from_points(points: &Vec<DataPoint>) -> Self {
        let initial = points
            .iter()
            .min_by(|a, b| a.time.cmp(&b.time))
            .unwrap()
            .time;

        Self {
            deltas: points
                .windows(2)
                .map(|a| {
                    let dist =
                        (f64::powf(a[1].x - a[0].x, 2.0) + f64::powf(a[1].y - a[0].y, 2.0)).sqrt();
                    let time_delta = (a[1].time - a[0].time).num_milliseconds() as f64;
                    DeltaTime {
                        time: (a[0].time - initial).num_milliseconds() as f64 + time_delta / 2.0, // assume this velocity occurs at the midpoint
                        rate: dist / time_delta,
                    }
                })
                .collect::<Vec<_>>(),
        }
    }

    fn from_self(deltas: &DeltaTimes) -> Self {
        Self {
            deltas: deltas
                .deltas
                .windows(2)
                .map(|a| {
                    let delta_vel = a[1].rate - a[0].rate;
                    let time_delta = a[1].time - a[0].time;
                    DeltaTime {
                        time: a[0].time + time_delta / 2.0, // assume this accelations occurs at the midpoint (so at x1)
                        rate: delta_vel / time_delta as f64,
                    }
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn graph(
        &self,
        backend: &DrawingArea<BitMapBackend<plotters::backend::RGBPixel>, plotters::coord::Shift>,
        color: RGBColor,
    ) -> Result<(), anyhow::Error> {
        let mut chart = ChartBuilder::on(backend)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .margin(50)
            .build_cartesian_2d(
                0.0..(self.deltas.last().unwrap().time - self.deltas[0].time) as f64,
                self.deltas
                    .iter()
                    .min_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap())
                    .unwrap()
                    .rate
                    ..self
                        .deltas
                        .iter()
                        .max_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap())
                        .unwrap()
                        .rate,
            )?;
        chart.plotting_area().fill(&TRANSPARENT)?;
        chart.configure_mesh().draw()?;

        chart.draw_series(
            self.deltas
                .iter()
                .map(|DeltaTime { time, rate, .. }| Circle::new((*time, *rate), 8, color.filled())),
        )?;

        // chart.draw_series(LineSeries::new(
        //     self.deltas
        //         .iter()
        //         .map(|DeltaTime { time, rate, .. }| (*time, *rate)),
        //     &color,
        // ))?;

        Ok(())
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
            y: data[2].parse::<f64>()?,
        })
    }
}
