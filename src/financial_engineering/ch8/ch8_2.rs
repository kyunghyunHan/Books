use chrono::NaiveDateTime;
use plotters::prelude::*;
use plotters::chart::SeriesLabelPosition;
use polars::prelude::*;
use std::error::Error;

const OUT_FILE_NAME: &str = "./assets/img/stock_subplots.png";

pub fn main() -> Result<(), Box<dyn Error>> {
    // 데이터 로드
    let df = LazyCsvReader::new("./assets/data/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?
        .collect()?;
    
    println!("Columns in DataFrame: {:?}", df.get_column_names());
    println!("DataFrame Schema: {:?}", df.schema());
    
    // 날짜 데이터 타입 확인
    let date_col = df.column("Date")?;
    println!("Date column type: {:?}", date_col.dtype());
    
    // Date 값을 문자열로 출력하여 확인
    println!("First few Date values:");
    for i in 0..5.min(df.height()) {
        let row = df.get(i);
        if let Some(row_values) = row {
            if !row_values.is_empty() {
                println!("{}: {:?}", i, row_values[0]);
            }
        }
    }
    
    // 날짜 데이터 수동 변환 시도
    let mut dates = Vec::new();
    for i in 0..df.height() {
        if let Some(row_values) = df.get(i) {
            if !row_values.is_empty() {
                // AnyValue를 문자열로 변환
                let date_str = format!("{}", row_values[0]);
                dates.push(date_str);
            } else {
                dates.push("".to_string());
            }
        } else {
            dates.push("".to_string());
        }
    }
    
    println!("First 5 converted dates: {:?}", dates.iter().take(5).collect::<Vec<_>>());
    
    // 서브플롯에 표시할 열 목록
    let columns_to_plot = [
        "AAPL.O", "MSFT.O", "INTC.O", "AMZN.O", "GS.N", 
        "SPY", ".SPX", ".VIX", "EUR=", "XAU=", "GDX", "GLD"
    ];
    
    // 이미지 크기 및 영역 설정
    let root = BitMapBackend::new(OUT_FILE_NAME, (1000, 1500)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // 차트를 그릴 영역 분할
    let areas = root.split_evenly((columns_to_plot.len(), 1));
    
    // 컬럼 이름 리스트를 String으로 변환
    let df_columns: Vec<String> = df.get_column_names()
        .iter()
        .map(|&col| col.to_string())
        .collect();
    
    for (idx, &col_name) in columns_to_plot.iter().enumerate() {
        // 컬럼이 존재하는지 확인
        if !df_columns.contains(&col_name.to_string()) {
            println!("Column {} not found, skipping", col_name);
            continue;
        }
        
        // 컬럼 데이터 가져오기 및 처리
        let column = df.column(col_name)?;
        println!("Column {} type: {:?}", col_name, column.dtype());
        
        // 컬럼 인덱스 찾기
        let col_idx = df.get_column_names().iter().position(|&c| c == col_name)
            .ok_or_else(|| format!("Column {} not found", col_name))?;
        
        // 데이터 변환 및 NA 값 필터링
        let mut valid_data: Vec<(usize, f64)> = Vec::new();
        
        for i in 0..df.height() {
            if let Some(row_values) = df.get(i) {
                if row_values.len() > col_idx {
                    let value_str = format!("{}", row_values[col_idx]);
                    if let Ok(v) = value_str.parse::<f64>() {
                        valid_data.push((i, v));
                    }
                }
            }
        }
        
        if valid_data.is_empty() {
            println!("No valid data for {}, skipping", col_name);
            continue;
        }
        
        // 데이터 범위 계산
        let min_val = valid_data.iter().map(|&(_, v)| v).fold(f64::INFINITY, |a, b| a.min(b));
        let max_val = valid_data.iter().map(|&(_, v)| v).fold(f64::NEG_INFINITY, |a, b| a.max(b));
        
        // 범위에 약간의 여유 추가
        let margin = (max_val - min_val) * 0.1;
        let y_range = (min_val - margin)..(max_val + margin);
        
        // 날짜 범위 설정 (인덱스 기반)
        let x_range = 0..valid_data.len();
        
        // 서브플롯 생성
        let mut chart = ChartBuilder::on(&areas[idx])
            .margin(5)
            .margin_left(40)  // 왼쪽 여백 늘리기
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(x_range.clone(), y_range)?;
        
        // 색상 선택 - 각 차트마다 다른 색상
        let colors = [
            &BLUE, &GREEN, &RED, &MAGENTA, &YELLOW, 
            &CYAN, &BLACK, &RGBColor(34, 139, 34), &RGBColor(128, 0, 128),
            &RGBColor(218, 165, 32), &RGBColor(210, 105, 30), &RGBColor(70, 130, 180)
        ];
        
        let color = colors[idx % colors.len()];
        
        // 격자 및 레이블 설정
        chart
            .configure_mesh()
            .light_line_style(&TRANSPARENT)
            .bold_line_style(RGBColor(200, 200, 200).mix(0.3))
            .y_labels(5)
            .x_labels(5)
            .x_label_formatter(&|x| {
                // 단순화: 데이터 길이에 따라 몇 개의 연도만 표시
                if *x == 0 || *x == x_range.end / 2 || *x == x_range.end - 1 {
                    let idx = valid_data.get(*x).map(|&(i, _)| i).unwrap_or(0);
                    if idx < dates.len() {
                        let date_str = &dates[idx];
                        if date_str.len() >= 4 {
                            return date_str[0..4].to_string();
                        }
                    }
                }
                "".to_string()
            })
            .draw()?;
        
        // 차트 제목 추가
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .background_style(WHITE.mix(0.8))
            .border_style(&BLACK)
            .label_font(("sans-serif", 18))
            .draw()?;
        
        // 선 그래프 그리기
        chart
            .draw_series(LineSeries::new(
                valid_data.iter().enumerate().map(|(idx, &(_, v))| (idx, v)),
                color.to_owned().mix(0.9).stroke_width(2),
            ))?
            .label(col_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2)));
    }
    
    // 이미지 저장
    root.present()?;
    println!("Chart saved to {}", OUT_FILE_NAME);
    
    Ok(())
}