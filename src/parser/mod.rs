mod data;
mod errors;
mod reader;

use chrono::NaiveDateTime;
pub use data::*;
pub use errors::*;
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
}

#[derive(Debug)]
pub struct DataPoint {
    pub time: NaiveDateTime,
    pub x: f64,
    pub y: f64,
}

const TIME_FORMAT: &str = "%d %b %Y %H:%M:%S:%3f";
const EOF_MARKER: &str = "$EOF$";
