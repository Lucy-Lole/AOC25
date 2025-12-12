use std::{fs};

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), &'static str> {
    // Change this to switch day
    let day = "day3";

    let file = match fs::read_to_string(format!("./src/inputs/{day}.txt")) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading input file: {e:?}"),
    };

    let day_result = match day {
        "day1" => day1::run_day_one(file),
        "day2" => day2::run_day_two(file),
        "day3" => day3::run_day_three(file),
        _ => Err(anyhow::anyhow!("Invalid day"))
    };

    match day_result {
        Ok(result) => println!("Success: {result}"),
        Err(err) => println!("Error: {err}")
    };

    return Ok(());
}
