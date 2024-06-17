mod parser;
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
    RGBColor(r,g,b)
}


fn main() -> Result<(), anyhow::Error> {
    let data = DataFile::create_from_files(&std::env::args().skip(1).collect()).unwrap();

    let root = BitMapBackend::new(
        "output.png",
        (4000, 4000),
    )
    .into_drawing_area();
    root.fill(&BLACK)?;

    for truck in data.trucks {

let color = generate_random_rgb();

    let region = &truck.timeframes[0];
    let ((x_min, x_max), (y_min, y_max)) = region.get_range();
    let mut chart = ChartBuilder::on(&root).build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    chart.draw_series(
        truck
            .timeframes
            .iter()
            .map(|a| a.points.iter())
            .flatten()
            .map(|DataPoint { x, y, .. }| Circle::new((*x, *y), 2, color.filled())),
    )?;
    chart.configure_series_labels().draw()?;
    }
    todo!();
}
