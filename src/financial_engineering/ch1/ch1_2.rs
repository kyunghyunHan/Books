use plotters::prelude::*;
use polars::prelude::*;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    // 1. 데이터 로드
    let df = LazyCsvReader::new("./assets/tr_eikon_eod_data.csv")
        .with_has_header(true)
        .finish()?;

    // 2. SPX 컬럼만 선택하고 NA 제거
    let df = df.select([col(".SPX")]).drop_nulls(None);

    // 3. Returns 계산 - log(price_t / price_t-1)
    let returns = df
        .select([
            col(".SPX").alias("price"),
            (col(".SPX") / col(".SPX").shift(lit(1))).alias("rets"),
        ])
        .collect()?;
    println!("{}", returns);
    // // 4. Volatility 계산 - rolling std * sqrt(252)
    // let volatility = returns.select([
    //     col("price"),
    //     col("rets"),
    //     col("rets")
    //         .rolling_std(
    //             RollingOptions {
    //                 window_size: 252,
    //                 min_periods: 1,
    //                 ..Default::default()
    //             }
    //         )
    //         .mul(lit(252.0).sqrt())
    //         .alias("vola")
    // ]);

    // // 결과를 DataFrame으로 변환
    // let result = volatility.collect()?;

    // // 결과 출력
    // println!("First few rows of the result:");
    // println!("{:?}", result.head(Some(5)));

    // // 5. 시각화
    // let root = BitMapBackend::new("financial_plots.png", (1000, 600))
    //     .into_drawing_area();
    // root.fill(&WHITE)?;

    // let (upper, lower) = root.split_vertically(300);

    // // Price 차트
    // let price_data: Vec<(f64, f64)> = result
    //     .column("price")?
    //     .f64()?
    //     .into_iter()
    //     .enumerate()
    //     .filter_map(|(i, v)| Some((i as f64, v?)))
    //     .collect();

    // let mut chart = ChartBuilder::on(&upper)
    //     .caption("Price", ("serif", 30))
    //     .margin(5)
    //     .x_label_area_size(30)
    //     .y_label_area_size(50)
    //     .build_cartesian_2d(
    //         0f64..price_data.len() as f64,
    //         result.column("price")?.min()?.try_into()? ..
    //         result.column("price")?.max()?.try_into()?
    //     )?;

    // chart.configure_mesh().draw()?;
    // chart.draw_series(LineSeries::new(price_data, &BLUE))?;

    // // Volatility 차트
    // let vola_data: Vec<(f64, f64)> = result
    //     .column("vola")?
    //     .f64()?
    //     .into_iter()
    //     .enumerate()
    //     .filter_map(|(i, v)| Some((i as f64, v?)))
    //     .collect();

    // let mut chart = ChartBuilder::on(&lower)
    //     .caption("Volatility", ("serif", 30))
    //     .margin(5)
    //     .x_label_area_size(30)
    //     .y_label_area_size(50)
    //     .build_cartesian_2d(
    //         0f64..vola_data.len() as f64,
    //         result.column("vola")?.min()?.try_into()? ..
    //         result.column("vola")?.max()?.try_into()?
    //     )?;

    // chart.configure_mesh().draw()?;
    // chart.draw_series(LineSeries::new(vola_data, &RED))?;

    Ok(())
}
