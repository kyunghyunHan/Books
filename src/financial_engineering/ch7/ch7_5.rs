use ndarray_rand::rand::rngs::StdRng;
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand::RngCore; // RngCore 트레이트 추가
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;
use ndarray::Array2;
use plotters::prelude::*;
use plotters::style::RGBColor;

// 색상 매핑을 위한 함수 (coolwarm 컬러맵 모방)
fn get_color(value: f64, min_val: f64, max_val: f64) -> RGBColor {
    // 값을 0~1 범위로 정규화
    let normalized = (value - min_val) / (max_val - min_val);
    
    // coolwarm 컬러맵 근사
    if normalized < 0.5 {
        // 파란색에서 흰색으로 (0~0.5)
        let intensity = (normalized * 2.0) as f64;
        RGBColor(
            (intensity * 255.0) as u8,
            (intensity * 255.0) as u8,
            255,
        )
    } else {
        // 흰색에서 빨간색으로 (0.5~1.0)
        let intensity = ((normalized - 0.5) * 2.0) as f64;
        RGBColor(
            255,
            ((1.0 - intensity) * 255.0) as u8,
            ((1.0 - intensity) * 255.0) as u8,
        )
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 랜덤 시드 설정
    let mut rng = StdRng::seed_from_u64(42);
    
    // 2차원 정규 분포 데이터 생성 (500개 포인트)
    let normal_dist = Normal::new(0.0, 1.0)?;
    let data: Array2<f64> = Array2::random_using((500, 2), normal_dist, &mut rng);
    
    // 색상 값 생성 (0~9 범위의 정수), NumPy와 유사하게
    let mut color_values = Vec::with_capacity(data.nrows());
    let mut rng2 = StdRng::seed_from_u64(123); // 다른 시드 사용
    for _ in 0..data.nrows() {
        let rand_val = (rng2.next_u32() % 10) as f64;
        color_values.push(rand_val);
    }
    
    // 색상 값의 최소, 최대값 찾기
    let min_c = 0.0;
    let max_c = 9.0;
    
    // 그래프 생성
    let root = BitMapBackend::new("colored_scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // 축 범위 설정
    let x_range = -3.5..3.5;
    let y_range = -4.0..3.5;
    
    // 컬러바 영역을 위해 메인 차트 영역과 컬러바 영역 분리
    let (main_area, color_bar_area) = root.split_horizontally(700);
    
    // 메인 차트 설정
    let mut chart = ChartBuilder::on(&main_area)
        .margin(10)
        .caption("Scatter Plot", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(x_range, y_range)?;
    
    // 격자 및 라벨 설정
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("1st")
        .y_desc("2nd")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 15))
        .draw()?;
    
    // 컬러맵이 적용된 산점도 그리기
    for i in 0..data.nrows() {
        let color = get_color(color_values[i], min_c, max_c);
        chart.draw_series(std::iter::once(
            Circle::new((data[[i, 0]], data[[i, 1]]), 5, color.filled())
        ))?;
    }
    
    // 컬러바 그리기
    let mut color_bar = ChartBuilder::on(&color_bar_area)
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .build_cartesian_2d(0..1, min_c..max_c)?;
    
    color_bar.configure_mesh()
        .disable_x_mesh()
        .disable_x_axis()
        .y_labels(10)
        .y_label_style(("sans-serif", 15))
        .draw()?;
    
    // 컬러바 채우기
    const COLORBAR_SEGMENTS: usize = 100;
    for i in 0..COLORBAR_SEGMENTS {
        let y_min = min_c + (max_c - min_c) * (i as f64 / COLORBAR_SEGMENTS as f64);
        let y_max = min_c + (max_c - min_c) * ((i + 1) as f64 / COLORBAR_SEGMENTS as f64);
        let color = get_color(y_min, min_c, max_c);
        
        color_bar.draw_series(std::iter::once(
            Rectangle::new([(0, y_min), (1, y_max)], color.filled())
        ))?;
    }
    
    root.present()?;
    println!("Plot has been saved to colored_scatter_plot.png");
    
    Ok(())
}