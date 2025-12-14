use std::{fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> Result<(), &'static str> {
    // Change this to switch day
    let day = "day8";

    let file = match fs::read_to_string(format!("./src/inputs/{day}.txt")) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading input file: {e:?}"),
    };

    let day_result = match day {
        "day1" => day1::run_day_one(file),
        "day2" => day2::run_day_two(file),
        "day3" => day3::run_day_three(file),
        "day4" => day4::run_day_four(file),
        "day5" => day5::run_day_five(file),
        "day6" => day6::run_day_six(file),
        "day7" => day7::run_day_seven(file),
        "day8" => day8::run_day_eight(file),
        _ => Err(anyhow::anyhow!("Invalid day"))
    };

    match day_result {
        Ok(result) => println!("Success: {result}"),
        Err(err) => println!("Error: {err}")
    };

    return Ok(());
}
