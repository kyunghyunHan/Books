use chrono::NaiveDate;
use plotters::prelude::*;
use plotters::chart::SeriesLabelPosition;
use polars::prelude::*;
use std::error::Error;

const OUT_FILE_NAME: &str = "./assets/img/ch8_3.png";

pub fn main() -> Result<(), Box<dyn Error>> {
    // 데이터 로드
    let df = LazyCsvReader::new("./assets/data/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?
        .collect()?;
    
    println!("Columns in DataFrame: {:?}", df.get_column_names());
    
    // 날짜 형식 확인을 위해 첫 몇 개의 날짜 출력
    let date_col = df.column("Date")?;
    println!("Date column type: {:?}", date_col.dtype());
    println!("First 5 date values:");
    for i in 0..5.min(df.height()) {
        if let Some(row) = df.get(i) {
            let date_idx = df.get_column_names()
                .iter()
                .position(|c| c.to_string() == "Date")
                .unwrap();
            println!("Date at row {}: '{}'", i, row[date_idx]);
        }
    }
    
    // 서브플롯에 표시할 열 목록 (이미지에 표시된 순서와 동일하게)
    let columns_to_plot: [&str; 12] = [
        "AAPL.O", "MSFT.O", "INTC.O", "AMZN.O", "GS.N", 
        "SPY", ".SPX", ".VIX", "EUR=", "XAU=", "GDX", "GLD"
    ];
    
    // 날짜 컬럼 가져오기
    let mut dates: Vec<NaiveDate> = Vec::new();
    let date_col = df.column("Date")?;
    
    // 날짜 컬럼 인덱스 가져오기
    let date_idx = df.get_column_names()
        .iter()
        .position(|c| c.to_string() == "Date")
        .ok_or("Date column not found")?;
    
    for i in 0..df.height() {
        if let Some(row) = df.get(i) {
            let date_value = &row[date_idx];
            let date_str = format!("{}", date_value);
            
            // 따옴표 제거 ('"2010-01-01"' -> '2010-01-01')
            let clean_date_str = date_str.trim().trim_matches('"');
            
            if let Ok(date) = NaiveDate::parse_from_str(clean_date_str, "%Y-%m-%d") {
                dates.push(date);
            } else {
                println!("Failed to parse date: \"{}\"", clean_date_str);
                dates.push(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            }
        }
    }
    
    // 이미지 영역 설정 - 이미지 2와 비슷한 크기로
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // 모든 자산의 로그 수익률과 누적 수익률을 계산
    let mut cumulative_returns: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut min_cum_return = f64::INFINITY;
    let mut max_cum_return = f64::NEG_INFINITY;
    
    for &col_name in columns_to_plot.iter() {
        // 컬럼 이름을 문자열로 변환하여 비교
        let col_exists = df.get_column_names()
            .iter()
            .any(|c| c.to_string() == col_name);
            
        if !col_exists {
            println!("Column {} not found, skipping", col_name);
            continue;
        }
        
        // 열 인덱스 찾기
        let col_idx = df.get_column_names()
            .iter()
            .position(|c| c.to_string() == col_name)
            .ok_or_else(|| format!("Column {} not found", col_name))?;
        
        // DataFrame에서 직접 값을 추출
        let mut values: Vec<f64> = Vec::new();
        for i in 0..df.height() {
            if let Some(row) = df.get(i) {
                let value_str = format!("{}", row[col_idx]);
                if let Ok(v) = value_str.parse::<f64>() {
                    values.push(v);
                } else {
                    values.push(f64::NAN);
                }
            }
        }
        
        // 로그 수익률 계산 (Python의 np.log(data / data.shift(1))와 같음)
        let mut log_returns: Vec<f64> = Vec::new();
        log_returns.push(0.0); // 첫 번째 값은 NaN이므로 0으로 설정
        
        for i in 1..values.len() {
            if values[i].is_nan() || values[i-1].is_nan() || values[i-1] == 0.0 {
                log_returns.push(0.0);
            } else {
                let log_return = (values[i] / values[i-1]).ln();
                log_returns.push(log_return);
            }
        }
        
        // 누적 수익률 계산 (Python의 cumsum()과 같음)
        let mut cum_return = 0.0;
        let mut series_returns: Vec<(f64, f64)> = Vec::new();
        
        for (i, &log_ret) in log_returns.iter().enumerate() {
            if !log_ret.is_nan() {
                cum_return += log_ret;
            }
            
            if i < dates.len() {
                // chrono 버전에 따라 아래 두 방법 중 하나를 사용
                #[cfg(feature = "modern_chrono")]
                let date_num = dates[i].and_hms_opt(0, 0, 0).unwrap().timestamp() as f64 / 86400.0;
                
                #[cfg(not(feature = "modern_chrono"))]
                let date_num = dates[i].and_hms(0, 0, 0).timestamp() as f64 / 86400.0;
                
                series_returns.push((date_num, cum_return));
                
                // 누적 수익률의 최소/최대값 업데이트
                min_cum_return = min_cum_return.min(cum_return);
                max_cum_return = max_cum_return.max(cum_return);
            }
        }
        
        cumulative_returns.push(series_returns);
    }
    
    // 주요 날짜의 타임스탬프 계산 (x축 레이블용)
    let years = ["2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018"];
    // chrono 버전에 따라 다른 방식으로 타임스탬프 계산
    #[cfg(feature = "modern_chrono")]
    let year_timestamps: Vec<f64> = years.iter()
        .map(|&year| {
            NaiveDate::parse_from_str(&format!("{}-01-01", year), "%Y-%m-%d")
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .timestamp() as f64 / 86400.0
        })
        .collect();
        
    #[cfg(not(feature = "modern_chrono"))]
    let year_timestamps: Vec<f64> = years.iter()
        .map(|&year| {
            NaiveDate::parse_from_str(&format!("{}-01-01", year), "%Y-%m-%d")
                .unwrap()
                .and_hms(0, 0, 0)
                .timestamp() as f64 / 86400.0
        })
        .collect();
    
    // Y축 범위 지정 (여백 추가)
    let margin = (max_cum_return - min_cum_return) * 0.1;
    let y_range = (min_cum_return - margin)..(max_cum_return + margin);
    
    // X축 범위 지정 (이제 날짜 대신 인덱스 사용)
    let x_range = 0.0..(df.height() as f64);
    
    // 차트 생성
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .margin_left(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)?;
    
    // 격자 및 레이블 설정
    chart
        .configure_mesh()
        .x_labels(9)  // 레이블 갯수
        .x_label_formatter(&|&x| {
            // 간단하게 연도만 표시
            let years = ["2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018"];
            let idx = (x / (df.height() as f64) * years.len() as f64) as usize;
            if idx < years.len() {
                years[idx].to_string()
            } else {
                "".to_string()
            }
        })
        .y_desc("Cumulative Log Return")
        .draw()?;
    
    // 색상 선택 - 각 시리즈마다 다른 색상
    let colors = [
        &RED, &BLUE, &GREEN, &MAGENTA, &BLACK, 
        &CYAN, &RGBColor(128, 0, 0), &RGBColor(128, 128, 0),
        &RGBColor(218, 165, 32), &RGBColor(70, 130, 180),
        &RGBColor(210, 105, 30), &RGBColor(0, 100, 0)
    ];
    
    // 범례 설정
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 12))
        .draw()?;
    
    // 각 자산의 누적 수익률 그래프 그리기
    for (idx, series_data) in cumulative_returns.iter().enumerate() {
        let col_name = columns_to_plot[idx];
        let color = colors[idx % colors.len()];
        
        chart
            .draw_series(LineSeries::new(
                series_data.clone(),
                color.stroke_width(2),
            ))?
            .label(col_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2)));
    }
    
    // 이미지 저장
    root.present()?;
    println!("Cumulative returns chart saved to {}", OUT_FILE_NAME);
    
    Ok(())
}