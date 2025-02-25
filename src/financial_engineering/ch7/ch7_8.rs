// use ndarray_rand::rand::rngs::StdRng;
// use ndarray_rand::rand::SeedableRng;
// use ndarray_rand::rand_distr::Normal;
// use ndarray_rand::RandomExt;
// use plotters::prelude::*;
// use ndarray::Array1;
// use plotters::coord::types::RangedCoordf64;
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 랜덤 시드 설정
//     let mut rng1 = StdRng::seed_from_u64(42);
//     let mut rng2 = StdRng::seed_from_u64(123);
    
//     // 두 개의 정규 분포 데이터 생성 (각 1000개 포인트)
//     // 첫 번째 데이터셋: 평균 0, 표준편차 1
//     let normal_dist1 = Normal::new(0.0, 1.0)?;
//     let data1: Array1<f64> = Array1::random_using(1000, normal_dist1, &mut rng1);
    
//     // 두 번째 데이터셋: 평균 0, 표준편차 1 (비슷하게 설정)
//     let normal_dist2 = Normal::new(0.0, 1.0)?;
//     let data2: Array1<f64> = Array1::random_using(1000, normal_dist2, &mut rng2);
    
//     // 데이터 통계 계산
//     let calc_stats = |data: &Array1<f64>| {
//         let mut sorted_data = data.to_vec();
//         sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
//         let len = sorted_data.len();
//         let min = *sorted_data.first().unwrap_or(&0.0);
//         let max = *sorted_data.last().unwrap_or(&0.0);
//         let median = if len % 2 == 0 {
//             (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
//         } else {
//             sorted_data[len / 2]
//         };
        
//         // 1사분위수 (Q1)
//         let q1_idx = len / 4;
//         let q1 = if len % 4 == 0 {
//             (sorted_data[q1_idx - 1] + sorted_data[q1_idx]) / 2.0
//         } else {
//             sorted_data[q1_idx]
//         };
        
//         // 3사분위수 (Q3)
//         let q3_idx = 3 * len / 4;
//         let q3 = if 3 * len % 4 == 0 {
//             (sorted_data[q3_idx - 1] + sorted_data[q3_idx]) / 2.0
//         } else {
//             sorted_data[q3_idx]
//         };
        
//         (min, q1, median, q3, max)
//     };
    
//     let stats1 = calc_stats(&data1);
//     let stats2 = calc_stats(&data2);
    
//     // 그래프 생성
//     let root = BitMapBackend::new("boxplot.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;
    
//     // 차트 설정 (RangedCoord 사용)
//     let mut chart = ChartBuilder::on(&root)
//         .margin(10)
//         .caption("Boxplot", ("sans-serif", 30))
//         .set_label_area_size(LabelAreaPosition::Left, 60)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40)
//         .build_cartesian_2d(
//             (0..3).into_segmented(), // 범주형 X 축
//             RangedCoordf64::new(-4.0..3.0) // RangedCoordf64로 감싼 Y 축 범위
//         )?;
    
//     // 격자 설정
//     chart.configure_mesh()
//         .disable_x_mesh()
//         .x_labels(3)
//         .y_labels(10)
//         .x_desc("data set")
//         .y_desc("value")
//         .x_label_formatter(&|x| {
//             match x.index() {
//                 1 => "1st".to_string(),
//                 2 => "2nd".to_string(),
//                 _ => "".to_string(),
//             }
//         })
//         .axis_desc_style(("sans-serif", 15))
//         .draw()?;
    
//     // 박스플롯 데이터
//     let (min1, q1_1, median1, q3_1, max1) = stats1;
//     let (min2, q1_2, median2, q3_2, max2) = stats2;
    
//     // 1번째 박스플롯 (1st)
//     draw_boxplot(&mut chart, 1, stats1)?;
    
//     // 2번째 박스플롯 (2nd)
//     draw_boxplot(&mut chart, 2, stats2)?;
    
//     root.present()?;
//     println!("Plot has been saved to boxplot.png");
    
//     Ok(())
// }

// fn draw_boxplot<DB: DrawingBackend>(
//     chart: &mut ChartContext<DB, Cartesian2d<SegmentedValue<usize>, RangedCoordf64>>,
//     x_pos: usize,
//     stats: (f64, f64, f64, f64, f64)
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let (min, q1, median, q3, max) = stats;
    
//     // 박스 너비 설정
//     let box_width = 0.5;
//     let half_width = box_width / 2.0;
    
//     // IQR (Interquartile Range)
//     let iqr = q3 - q1;
    
//     // 위스커 경계 (최대 1.5 * IQR까지)
//     let lower_whisker = (min).max(q1 - 1.5 * iqr);
//     let upper_whisker = (max).min(q3 + 1.5 * iqr);
    
//     // 박스 그리기 (Rectangle)
//     chart.draw_series(std::iter::once(
//         Rectangle::new(
//             [
//                 (SegmentedValue::from(x_pos) - half_width, lower_whisker.into()),
//                 (SegmentedValue::from(x_pos) + half_width, upper_whisker.into())
//             ],
//             RGBColor(200, 200, 200).filled()
//         )
//     ))?;
    
//     // 중앙선 (중앙값) 그리기
//     chart.draw_series(std::iter::once(
//         PathElement::new(
//             vec![
//                 (SegmentedValue::from(x_pos) - half_width, median.into()),
//                 (SegmentedValue::from(x_pos) + half_width, median.into())
//             ],
//             BLACK.stroke_width(2)
//         )
//     ))?;
    
//     // 위스커 그리기 (세로선)
//     chart.draw_series(std::iter::once(
//         PathElement::new(
//             vec![
//                 (SegmentedValue::from(x_pos), lower_whisker.into()),
//                 (SegmentedValue::from(x_pos), q1.into())
//             ],
//             BLACK.stroke_width(1)
//         )
//     ))?;
    
//     chart.draw_series(std::iter::once(
//         PathElement::new(
//             vec![
//                 (SegmentedValue::from(x_pos), q3.into()),
//                 (SegmentedValue::from(x_pos), upper_whisker.into())
//             ],
//             BLACK.stroke_width(1)
//         )
//     ))?;
    
//     // 위스커 끝 가로선 그리기
//     chart.draw_series(std::iter::once(
//         PathElement::new(
//             vec![
//                 (SegmentedValue::from(x_pos) - 0.3, lower_whisker.into()),
//                 (SegmentedValue::from(x_pos) + 0.3, lower_whisker.into())
//             ],
//             BLACK.stroke_width(1)
//         )
//     ))?;
    
//     chart.draw_series(std::iter::once(
//         PathElement::new(
//             vec![
//                 (SegmentedValue::from(x_pos) - 0.3, upper_whisker.into()),
//                 (SegmentedValue::from(x_pos) + 0.3, upper_whisker.into())
//             ],
//             BLACK.stroke_width(1)
//         )
//     ))?;
    
//     Ok(())
// }