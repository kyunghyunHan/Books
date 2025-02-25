use chrono::NaiveDateTime;
use ndarray::Array2;
use polars::{lazy, prelude::*};
use rand::thread_rng;
use rand_distr::{Distribution, StandardNormal}; // Distribution trait을 추가로 import
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut df = df! [
        "index" => ["a", "b", "c", "d"],
        "numbers" => [10, 20, 30, 40],
    ]?;

    println!("{:?}", df.get_columns());

    println!("{:?}", df.get_column_names());
    //ioc
    let filtered = df
        .clone()
        .lazy()
        .clone()
        .filter(col("index").eq(lit("c")))
        .collect()?;

    println!("{:?}", filtered);
    //iloc [1:3]
    let sliced = df.clone().lazy().slice(1, 2).collect()?;
    println!("{:?}", sliced);
    //df.sum()
    let sum = df.clone().lazy().select([col("numbers").sum()]).collect()?;

    println!("{:?}", sum);
    // let doubled2 = df.apply("numbers", |flosts| flosts)?;
    //df.apply
    let doubled = df
        .clone()
        .lazy()
        .select([col("numbers").pow(2)])
        .collect()?;

    println!("{:?}", doubled);
    df.with_column(Series::new(
        "floats".into(),
        [1.5f64, 2.5f64, 3.5f64, 4.5f64],
    ))?;

    let new_row = DataFrame::new(vec![
        Series::new("index".into(), ["Jil"]).into(),
        Series::new("numbers".into(), &[100i32]).into(),
        Series::new("floats".into(), &[5.75f64]).into(),
    ])?;

    df.extend(&new_row)?;
    //df.mean()//평균
    println!("{:?}", df.clone().lazy().mean().collect()?);
    //std : 표준편차
    println!("{:?}", df.clone().lazy().std(1).collect()?);

    let mut rng = thread_rng();
    let normal = StandardNormal;
    let a: Array2<f64> = Array2::from_shape_fn((9, 4), |_| normal.sample(&mut rng));
    let columns: Vec<Column> = (0..a.ncols())
        .map(|i| Ok(Series::new(format!("col_{}", i).into(), a.column(i).to_vec()).into()))
        .collect::<PolarsResult<Vec<Column>>>()?;
    let mut df = DataFrame::new(columns)?;
    println!("{:?}", df.clone());
    let df = df
        .rename("col_0", "No1".into())?
        .rename("col_1", "No2".into())?
        .rename("col_2", "No3".into())?
        .rename("col_3", "No4".into())?;

    let mean_no2 = df.column("No2")?.mean_reduce();
   
    println!("Mean of No2: {:?}", mean_no2);

    let start = NaiveDateTime::parse_from_str("2019-01-31 00:00:00", "%Y-%m-%d %H:%M:%S")?;
    let end = NaiveDateTime::parse_from_str("2019-09-30 00:00:00", "%Y-%m-%d %H:%M:%S")?;

    let dates = date_range(
        "dates".into(),         // PlSmallStr로 변환
        start,                  // NaiveDateTime
        end,                    // NaiveDateTime
        Duration::parse("1mo"), // Duration
        ClosedWindow::Both,     // ClosedWindow
        TimeUnit::Milliseconds, // TimeUnit
        None,                   // timezone
    )?;
    let date_series = Series::new("dates".into(), dates);
    println!("{:?}", date_series);

    Ok(())
}
