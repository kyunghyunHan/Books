use chrono::NaiveDateTime;
use plotters::prelude::*;
use plotters::series::LineSeries;
use polars::prelude::*;
use std::error::Error;

const OUT_FILE_NAME: &str = "assets/stock.png";

pub fn main() -> Result<(), Box<dyn Error>> {
    let df = LazyCsvReader::new("./assets/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?;
    //dropna
    let df_check = LazyCsvReader::new("./assets/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?
        .collect()?;
    println!("Original Date format:");
    println!("{:?}", df_check.column("Date")?.head(Some(5)));
    let df = df.select([col("Date"), col(".SPX")]).drop_nulls(None);

    let dfs = df.clone().collect()?;
    println!("{:?}", dfs);

    //rts
    // 2. 수익률 계산 - returns로 이름 변경
    let returns = df.select([
        col("Date").str().strptime(
            DataType::Datetime(TimeUnit::Microseconds, None),
            StrptimeOptions {
                format: Some("%Y-%m-%d".into()), // 이 형식이 맞습니다
                strict: true,
                exact: true,
                cache: true,
            },
            col("Date"), // 원본 Date 컬럼을 참조
        ),
        col(".SPX").alias("price"),
        (col(".SPX") / col(".SPX").shift(lit(1)))
            .log(std::f64::consts::E)
            .cast(DataType::Float64)
            .alias("returns"), // 👈 rets를 returns로 변경
    ]);

    // 중간 결과 확인
    let returns_df = returns.clone().collect()?;
    println!("Returns DataFrame:");
    println!("Columns: {:?}", returns_df.get_column_names());
    println!("{:?}", returns_df.head(Some(5)));
    //rolling std:이동편차 구현
    // 3. 변동성 계산
    let volatility = returns_df.lazy().select([
        col("Date"),
        col("price"),
        (col("returns").rolling_std(RollingOptionsFixedWindow {
            window_size: 252,
            min_periods: 1,
            center: false,
            weights: None,
            fn_params: None,
        }) * lit(252.0_f64.sqrt()))
        .alias("vola"), // alias를 전체 expression 뒤로 이동
    ]);

    // 4. 결과 수집
    let result = volatility.collect()?;

    // 결과 확인 - 좀 더 읽기 쉽게 출력
    println!("\nFinal Result:");
    println!("Available columns: {:?}", result.get_column_names());
    println!("First few rows:");
    if let (Ok(date_col), Ok(price_col), Ok(vola_col)) = (
        result.column("Date"),
        result.column("price"),
        result.column("vola"),
    ) {
        // datetime과 f64 시리즈로 변환
        if let (Ok(dates), Ok(prices), Ok(volas)) =
            (date_col.datetime(), price_col.f64(), vola_col.f64())
        {
            // 처음 5개 행 출력
            for idx in 0..5 {
                let date_str = dates
                    .get(idx)
                    .map(|d| {
                        let secs = d / 1_000_000_000;
                        NaiveDateTime::from_timestamp_opt(secs, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();

                let price_str = prices
                    .get(idx)
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_default();

                let vola_str = volas
                    .get(idx)
                    .map(|v| format!("{:.6}", v))
                    .unwrap_or_default();

                println!(
                    "Date: {}, Price: {}, Volatility: {}",
                    date_str, price_str, vola_str
                );
            }
        }
    }
    // 결과 확인
    println!("\nFinal Result:");
    println!("Available columns: {:?}", result.get_column_names());
    println!("First few rows:\n{:?}", result.head(Some(5)));
    // 결과 확인
    println!("\nFinal Result:");
    println!("Available columns: {:?}", result.get_column_names());
    println!("First few rows:\n{:?}", result.head(Some(5)));

    // 5. 시각화 설정
    let root = BitMapBackend::new(OUT_FILE_NAME, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // 차트 레이아웃 선택 (true: 위아래, false: 좌우)
    let vertical_layout = true;
    let (first_area, second_area) = if vertical_layout {
        root.split_vertically(400)
    } else {
        root.split_horizontally(500)
    };

    // 날짜 데이터 준비

    let dates: Vec<i64> = result
        .column("Date")?
        .datetime()?
        .into_iter()
        .filter_map(|opt_date| opt_date)
        .map(|ts| ts * 1000) // microseconds를 nanoseconds로 변환
        .collect();

    if dates.is_empty() {
        return Err("No valid dates found".into());
    }
    let min_date = *dates.first().unwrap();
    let max_date = *dates.last().unwrap();

    // Price 차트 그리기
    let price_data: Vec<(i64, f64)> = dates
        .iter()
        .zip(result.column("price")?.f64()?.into_iter().filter_map(|v| v))
        .map(|(&date, price)| (date, price))
        .collect();

    let price_column = result.column("price")?.f64()?;
    let min_price = price_column.min().unwrap_or(f64::NAN);
    let max_price = price_column.max().unwrap_or(f64::NAN);

    let mut chart = ChartBuilder::on(&first_area)
        .caption("Price", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart
        .configure_mesh()
        .x_labels(20)
        .x_label_formatter(&|x| {
            let secs = x / 1_000_000_000; // nanoseconds to seconds
            NaiveDateTime::from_timestamp_opt(secs, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "N/A".to_string())
        })
        .draw()?;
    chart.draw_series(LineSeries::new(price_data, &BLUE))?;

    // Volatility 차트 그리기
    if let Ok(vola_column) = result.column("vola") {
        let vola_data: Vec<(i64, f64)> = dates
            .iter()
            .zip(vola_column.f64()?.into_iter().filter_map(|v| v))
            .map(|(date, vola)| (*date, vola))
            .collect();

        let min_vola = vola_column.f64()?.min().unwrap_or(f64::NAN);
        let max_vola = vola_column.f64()?.max().unwrap_or(f64::NAN);

        let mut chart = ChartBuilder::on(&second_area)
            .caption("Volatility", ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(min_date..max_date, min_vola..max_vola)?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| {
                let secs = x / 1_000_000_000; // nanoseconds to seconds
                let naive = NaiveDateTime::from_timestamp_opt(secs, 0).unwrap();
                naive.format("%Y-%m-%d").to_string()
            })
            .draw()?;
        chart.draw_series(LineSeries::new(vola_data, &RED))?;
    }

    root.present()?;
    Ok(())
}
