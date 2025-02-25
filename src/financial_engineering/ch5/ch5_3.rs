use ndarray::Array1;
use ndarray_rand::rand::rngs::StdRng; // StdRng 사용
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;
use plotters::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set random seed (ndarray-rand uses a different approach)
    let mut rng = StdRng::seed_from_u64(1000);

    // Generate random normal distribution (equivalent to np.random.standard_normal)
    let y: Array1<f64> = Array1::random_using(20, StandardNormal, &mut rng);

    // Create x array (equivalent to np.arange)
    let x: Array1<f64> = Array1::range(0., y.len() as f64, 1.);

    // Plot using plotters
    let root = BitMapBackend::new("./assets/img/plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_y = y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0f64..y.len() as f64, // x 범위 수정
            min_y..max_y,         // y 범위 수정
        )?;
    
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        x.iter().zip(y.iter()).map(|(x, y)| (*x, *y)),
        &BLUE,
    ))?;

    root.present()?;

    Ok(())
}
