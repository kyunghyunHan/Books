use ndarray::Array2;
use ndarray_rand::rand::rngs::StdRng;
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;
use plotters::prelude::*;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 랜덤 시드 설정
    let mut rng = StdRng::seed_from_u64(42);

    // 20x2 표준 정규 분포 생성 (np.random.standard_normal((20, 2)) 와 유사)
    let y: Array2<f64> = Array2::random_using((20, 2), StandardNormal, &mut rng);

    // cumsum 계산 (각 열에 대해)
    let mut y_cumsum1 = Vec::with_capacity(y.nrows());
    let mut y_cumsum2 = Vec::with_capacity(y.nrows());
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..y.nrows() {
        sum1 += y[[i, 0]];
        sum2 += y[[i, 1]];
        y_cumsum1.push(sum1);
        y_cumsum2.push(sum2);
    }

    // x 값 생성
    let x: Vec<f64> = (0..y.nrows()).map(|i| i as f64).collect();

    // 그래프 생성
    let root = BitMapBackend::new("assets/img/two_series_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 데이터의 최소/최대값 찾기 (두 시리즈 모두 고려)
    let min_y = y_cumsum1
        .iter()
        .chain(y_cumsum2.iter())
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = y_cumsum1
        .iter()
        .chain(y_cumsum2.iter())
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("A Simple Plot", ("sans-serif", 20))
        .build_cartesian_2d(0f64..19f64, (min_y - 0.5)..(max_y + 0.5))?;

    // 격자 및 라벨 설정
    chart
        .configure_mesh()
        .x_desc("Index")
        .y_desc("Value")
        .axis_desc_style(("sans-serif", 15))
        .light_line_style(&WHITE.mix(0.3))
        .draw()?;

    // 첫 번째 시리즈: 녹색 선과 빨간색 점
    chart.draw_series(LineSeries::new(
        x.iter().zip(y_cumsum1.iter()).map(|(x, y)| (*x, *y)),
        &GREEN,
    ))?;

    chart.draw_series(PointSeries::of_element(
        x.iter().zip(y_cumsum1.iter()).map(|(x, y)| (*x, *y)),
        5,
        &RED,
        &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
    ))?;

    // 두 번째 시리즈: 파란색 선과 빨간색 점
    chart.draw_series(LineSeries::new(
        x.iter().zip(y_cumsum2.iter()).map(|(x, y)| (*x, *y)),
        &BLUE,
    ))?;

    chart.draw_series(PointSeries::of_element(
        x.iter().zip(y_cumsum2.iter()).map(|(x, y)| (*x, *y)),
        5,
        &RED,
        &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
    ))?;

    root.present()?;
    println!("Plot has been saved to two_series_plot.png");

    Ok(())
}
