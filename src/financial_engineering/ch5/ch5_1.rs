use polars::{lazy, prelude::*};
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

    Ok(())
}
