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

    //df.apply
    let doubled = df
        .clone()
        .lazy()
        .select([col("numbers").pow(2)])
        .collect()?;

    println!("{:?}", doubled);

    Ok(())
}
