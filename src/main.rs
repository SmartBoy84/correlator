mod parser;
mod model;
use parser::*;
use rand::{self, Rng};

use plotters::prelude::*;

const data_dir: &str = "data";
const data_prefix: &str = "B4_truck";

fn generate_random_rgb() -> RGBColor {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    RGBColor(r, g, b)
}

fn main() -> Result<(), anyhow::Error> {
    let data = DataFile::create_from_files(&std::env::args().skip(1).collect()).unwrap();
    let truck = &data.trucks[0];
    let frame = &truck.timeframes[0];
    println!("{:?}", frame.acceleration);

    let correlate = BitMapBackend::new("valaccel.png", (2500, 2500)).into_drawing_area();
    correlate.fill(&WHITE)?;

    frame.acceleration.graph(&correlate, BLACK)?;
    frame.velocity.graph(&correlate, GREEN)?;

    // let truck = &data.trucks[12];
    let path_map = BitMapBackend::new("path_map.png", (2500, 2500)).into_drawing_area();

    let region = &truck.timeframes[0];
    let ((x_min, x_max), (y_min, y_max)) = region.get_range();
    let mut chart = ChartBuilder::on(&path_map).build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    // println!("{x_min} {x_max} {y_min} {y_max}");
    chart.draw_series(LineSeries::new(
        frame.points.iter().map(|DataPoint { x, y, .. }| (*x, *y)),
        WHITE,
    ))?;
    Ok(())
}
