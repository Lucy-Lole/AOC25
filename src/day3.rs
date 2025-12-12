use anyhow::Result;

struct Bank(Vec<u32>);

pub fn run_day_three(file_as_string: String) -> Result<String> {
    let banks = process_data(file_as_string)?;

    let result_value = total_joltage(banks);

    return Ok(format!("Result: {result_value}"));
}

fn get_joltage(bank: &Bank) -> u32 {
    let max_value = bank.0[0..bank.0.len()-1].iter().max().unwrap();

    let mut itt = bank.0.iter();
    let mut found = false;

    while !found {
        if itt.next() == Some(max_value) {
            found = true;
        }
    }

    let second_max = itt.max().unwrap();

    return (max_value * 10) + *second_max;
}

fn total_joltage(banks: Vec<Bank>) -> u32 {
    return banks.iter().fold(0, |acc, item| acc + get_joltage(item));
}

fn process_data(file: String) -> Result<Vec<Bank>> {
    return file
        .split('\n')
        .filter(|l| l.len() > 1)
        .map(|r| parse_bank(r))
        .collect();
}

fn parse_bank(line: &str) -> Result<Bank> {
    let bank: Vec<u32> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    return Ok(Bank(bank));
}