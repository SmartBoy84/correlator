mod data;
mod errors;
mod reader;

use chrono::NaiveDateTime;
pub use data::*;
pub use errors::*;
use plotters::style::RGBColor;
use rand::Rng;
pub use reader::*;

#[derive(Debug)]
pub struct DataFile {
    pub trucks: Vec<Truck>,
}

#[derive(Debug)]
pub struct Truck {
    pub name: String,
    pub timeframes: Vec<TimeFrame>,
}

#[derive(Debug)]
pub struct TimeFrame {
    pub points: Vec<DataPoint>,
    pub velocity: DeltaTimes,
    pub acceleration: DeltaTimes
}

#[derive(Debug)]
pub struct DeltaTimes {
    pub deltas: Vec<DeltaTime>
}

#[derive(Debug)]
pub struct DataPoint {
    pub time: NaiveDateTime,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct DeltaTime {
    pub time: f64,
    pub rate: f64
}

const TIME_FORMAT: &str = "%d %b %Y %H:%M:%S:%3f";
const EOF_MARKER: &str = "$EOF$";

fn generate_random_rgb() -> RGBColor {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    RGBColor(r, g, b)
}
