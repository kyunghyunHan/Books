use ndarray::Array2;
use ndarray_rand::rand::rngs::StdRng;
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;
use plotters::prelude::*;
const OUT_FILE_NAME: &str = "./assets/img/ch_7_4.png";

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 랜덤 시드 설정
    let mut rng = StdRng::seed_from_u64(42);

    // 2차원 정규 분포 데이터 생성 (1000개 포인트)
    // 평균 0, 표준편차 1인 정규분포 사용
    let normal_dist = Normal::new(0.0, 1.0)?;
    let data: Array2<f64> = Array2::random_using((1000, 2), normal_dist, &mut rng);

    // 그래프 생성
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 축 범위 설정 (-3.5 ~ 3.5)
    let x_range = -3.5..3.5;
    let y_range = -4.0..3.5;

    // 차트 설정
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Scatter Plot", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(x_range, y_range)?;

    // 격자 및 라벨 설정
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("1st")
        .y_desc("2nd")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 15))
        .draw()?;

    // 산점도 그리기
    chart.draw_series(
        data.rows()
            .into_iter()
            .map(|row| Circle::new((row[0], row[1]), 3, BLUE.filled())),
    )?;

    root.present()?;
    println!("Plot has been saved to scatter_plot.png");

    Ok(())
}
