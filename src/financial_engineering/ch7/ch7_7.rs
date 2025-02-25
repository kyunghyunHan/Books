use ndarray_rand::rand::rngs::StdRng;
use ndarray_rand::rand::SeedableRng;
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;
use plotters::prelude::*;
use ndarray::Array1;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 랜덤 시드 설정
    let mut rng1 = StdRng::seed_from_u64(42);
    let mut rng2 = StdRng::seed_from_u64(123);
    
    // 두 개의 정규 분포 데이터 생성 (각 1000개 포인트)
    // 첫 번째 데이터셋: 평균 0, 표준편차 1
    let normal_dist1 = Normal::new(0.0, 1.0)?;
    let data1: Array1<f64> = Array1::random_using(1000, normal_dist1, &mut rng1);
    
    // 두 번째 데이터셋: 평균 0.2, 표준편차 0.9 (약간 다르게 설정)
    let normal_dist2 = Normal::new(0.2, 0.9)?;
    let data2: Array1<f64> = Array1::random_using(1000, normal_dist2, &mut rng2);
    
    // 그래프 생성
    let root = BitMapBackend::new("histogram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // 데이터 범위 설정 (히스토그램을 위한)
    let min_val = -4.0;
    let max_val = 4.0;
    
    // 빈(bin) 설정
    let bin_count = 25;
    let bin_width = (max_val - min_val) / bin_count as f64;
    
    // 히스토그램 계산 (수동으로)
    let mut hist1 = vec![0; bin_count];
    let mut hist2 = vec![0; bin_count];
    
    // 첫 번째 데이터셋의 히스토그램 계산
    for &val in data1.iter() {
        if val >= min_val && val < max_val {
            let bin = ((val - min_val) / bin_width).floor() as usize;
            if bin < bin_count {
                hist1[bin] += 1;
            }
        }
    }
    
    // 두 번째 데이터셋의 히스토그램 계산
    for &val in data2.iter() {
        if val >= min_val && val < max_val {
            let bin = ((val - min_val) / bin_width).floor() as usize;
            if bin < bin_count {
                hist2[bin] += 1;
            }
        }
    }
    
    // 최대 빈도수 찾기 (y축 범위 설정용)
    let max_freq = hist1.iter().chain(hist2.iter()).max().unwrap_or(&0) + 5;
    
    // 차트 설정
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Histogram", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            min_val..max_val,
            0..(max_freq as i32)
        )?;
    
    // 격자 및 라벨 설정
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("value")
        .y_desc("frequency")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;
    
    // 첫 번째 히스토그램 막대 그리기 (파란색)
    chart.draw_series(
        hist1.iter().enumerate().map(|(i, &count)| {
            let x0 = min_val + i as f64 * bin_width;
            let x1 = x0 + bin_width * 0.9; // 너비를 조정하여 겹치도록 설정
            
            Rectangle::new(
                [(x0, 0), (x1, count)],
                BLUE.mix(0.7).filled()
            )
        })
    )?
    .label("1st")
    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], BLUE.mix(0.7).filled()));
    
    // 두 번째 히스토그램 막대 그리기 (녹색) - 첫 번째 히스토그램 위에 겹쳐 그리기
    chart.draw_series(
        hist2.iter().enumerate().map(|(i, &count)| {
            let x0 = min_val + i as f64 * bin_width;
            let x1 = x0 + bin_width * 0.9;
            
            Rectangle::new(
                [(x0, 0), (x1, count)],
                GREEN.mix(0.7).filled()
            )
        })
    )?
    .label("2nd")
    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], GREEN.mix(0.7).filled()));
    
    // 범례 그리기
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(BLACK)
        .draw()?;
    
    root.present()?;
    println!("Plot has been saved to histogram.png");
    
    Ok(())
}