use anyhow::Result;

type Bank = Vec<u64>;

pub fn run_day_three(file_as_string: String) -> Result<String> {
    let banks = process_data(file_as_string)?;

    let result_value = total_joltage(banks);

    return Ok(format!("Result: {result_value}"));
}

fn get_joltage(bank: Bank, reserve: u32) -> u64 {
    if reserve == 0 {
        return *bank.iter().max().unwrap();
    }

    let max_value = bank[0..bank.len() - (reserve as usize)].iter().max().unwrap();

    let mut itt = bank.iter();
    let mut found = false;

    while !found {
        if itt.next() == Some(max_value) {
            found = true;
        }
    }

    let remaining: Vec<u64> = itt.map(|a| a.to_owned()).collect();

    let base: u64 = 10;
    return (max_value * base.pow(reserve)) + get_joltage(remaining, reserve-1);
}

fn total_joltage(banks: Vec<Bank>) -> u64 {
    return banks.iter().fold(0, |acc, item| acc + get_joltage(item.to_owned(), 11));
}

fn process_data(file: String) -> Result<Vec<Bank>> {
    return file
        .split('\n')
        .filter(|l| l.len() > 1)
        .map(|r| parse_bank(r))
        .collect();
}

fn parse_bank(line: &str) -> Result<Bank> {
    let bank: Vec<u64> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    return Ok(bank);
}