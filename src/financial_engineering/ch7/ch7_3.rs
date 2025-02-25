use ndarray::Array1;
use plotters::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 첫 번째 그래프 데이터 (감소하는 선 그래프)
    let y1: Vec<f64> = vec![
        -30.0, 0.0, 20.0, -40.0, -100.0, -120.0, -170.0, -190.0, -280.0, -330.0, -180.0, -290.0,
        -440.0, -430.0, -250.0, -360.0, -270.0, -220.0,
    ];

    // 두 번째 그래프 데이터 (막대 그래프, 증가하는 형태)
    let y2: Vec<f64> = vec![
        0.1, 0.0, 2.3, 3.6, 3.7, 5.1, 5.5, 6.7, 7.1, 6.8, 6.1, 7.0, 6.8, 6.5, 6.2, 6.8, 5.1, 5.0,
        6.0, 4.8,
    ];

    // x 값 생성
    let x1: Vec<f64> = (0..y1.len()).map(|i| i as f64).collect();
    let x2: Vec<usize> = (0..y2.len()).collect();

    // 그래프 생성을 위한 영역 설정
    let root =
        BitMapBackend::new("assets/img/subplots_customized.png", (900, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    // 영역을 두 부분으로 나누기 (좌우로)
    let areas = root.split_evenly((1, 2));

    // 첫 번째 서브플롯 (선 그래프 + 점 그래프)
    {
        let mut chart = ChartBuilder::on(&areas[0])
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("1st Data Set", ("sans-serif", 20))
            .build_cartesian_2d(0f64..(y1.len() - 1) as f64, -450.0..50.0)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("index")
            .y_desc("value")
            .axis_desc_style(("sans-serif", 15))
            .label_style(("sans-serif", 15))
            .draw()?;

        // 파란색 선 그리기
        chart
            .draw_series(LineSeries::new(
                x1.iter().zip(y1.iter()).map(|(x, y)| (*x, *y)),
                &BLUE,
            ))?
            .label("1st")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        // 빨간색 원형 마커 추가
        chart.draw_series(PointSeries::of_element(
            x1.iter().zip(y1.iter()).map(|(x, y)| (*x, *y)),
            7,
            &RED,
            &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
        ))?;

        // 범례 그리기
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(WHITE)
            .border_style(BLACK)
            .draw()?;
    }

    // 두 번째 서브플롯 (막대 그래프)
    {
        let mut chart = ChartBuilder::on(&areas[1])
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("2nd Data Set", ("sans-serif", 20))
            .build_cartesian_2d(0..y2.len(), 0.0..8.0)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("index")
            .y_desc("value")
            .axis_desc_style(("sans-serif", 15))
            .label_style(("sans-serif", 15))
            .draw()?;

        // 녹색 막대 그래프 그리기
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(GREEN)
                    .margin(15)
                    .data(x2.iter().zip(y2.iter()).map(|(&i, &v)| (i, v))),
            )?
            .label("2nd")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], GREEN));

        // 범례 그리기
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .background_style(WHITE)
            .border_style(BLACK)
            .draw()?;
    }

    // 그림 하단에 캡션 추가
    // (plotters에서는 이미지 외부 캡션을 직접 지원하지 않으므로, 실제 구현 시에는 이미지 편집 도구를 사용해야 할 수 있음)

    root.present()?;
    println!("Plot has been saved to subplots_customized.png");

    Ok(())
}
