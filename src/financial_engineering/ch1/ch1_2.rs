use plotters::prelude::*;
use plotters::series::LineSeries;
use polars::prelude::*;
use std::error::Error;
const OUT_FILE_NAME: &str = "assets/stock.png";

pub fn main() -> Result<(), Box<dyn Error>> {
    let df = LazyCsvReader::new("./assets/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?;

    let df = df.select([col("Date"), col(".SPX")]).drop_nulls(None);

    // 2. 수익률 계산 - returns로 이름 변경
    let returns = df.select([
        col("Date"),
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

    // 3. 변동성 계산
    let volatility = returns_df.lazy().select([
        col("Date"),
        col("price"),
        col("returns") // returns 컬럼 한 번만 선택
            .rolling_std(RollingOptionsFixedWindow {
                window_size: 252,
                min_periods: 1,
                center: false,
                weights: None,
                fn_params: None,
            })
            * (lit(252.0_f64.sqrt())).alias("vola"), // 변동성 결과만 별도 컬럼으로 저장
    ]);

    // 4. 결과 수집
    let result = volatility.collect()?;

    // 결과 확인
    println!("\nFinal Result:");
    println!("Available columns: {:?}", result.get_column_names());
    println!("First few rows:\n{:?}", result.head(Some(5)));
    // 결과 확인
    println!("\nFinal Result:");
    println!("Available columns: {:?}", result.get_column_names());
    println!("First few rows:\n{:?}", result.head(Some(5)));

    // 5. 시각화 - x축을 날짜로 사용
    let mut buffer = vec![0u8; 1000 * 600 * 3];
    let root = BitMapBackend::new(OUT_FILE_NAME, (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(300);

    // 날짜를 인덱스로 변환
    let price_data: Vec<(f64, f64)> = result
        .column("price")?
        .f64()?
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| Some((i as f64, v?)))
        .collect();

    let price_column = result.column("price")?.f64()?;
    let min_price = price_column.min().unwrap_or(f64::NAN);
    let max_price = price_column.max().unwrap_or(f64::NAN);

    // Price 차트 그리기
    let mut chart = ChartBuilder::on(&upper)
        .caption("Price", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..price_data.len() as f64, min_price..max_price)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(price_data, &BLUE))?;

    // Volatility 차트 그리기 (vola 컬럼이 있는 경우에만)
    if let Ok(vola_column) = result.column("vola") {
        let vola_data: Vec<(f64, f64)> = vola_column
            .f64()?
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| Some((i as f64, v?)))
            .collect();

        let min_vola = vola_column.f64()?.min().unwrap_or(f64::NAN);
        let max_vola = vola_column.f64()?.max().unwrap_or(f64::NAN);

        let mut chart = ChartBuilder::on(&lower)
            .caption("Volatility", ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..vola_data.len() as f64, min_vola..max_vola)?;

        chart.configure_mesh().draw()?;
        chart.draw_series(LineSeries::new(vola_data, &RED))?;
    }

    root.present()?;

    Ok(())
}
