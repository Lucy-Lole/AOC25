use anyhow::Result;

struct ProductId {
    first: i64,
    last: i64
}

pub fn run_day_two(file_as_string: String) -> Result<String> {
    let product_ids = process_data(file_as_string)?;

    let result_value = process_ids(product_ids);

    return Ok(format!("Result: {result_value}"));
}

fn process_ids(ids: Vec<ProductId>) -> i64 {
    let mut count_of_invalid = 0;

    for product_id in ids {
        for i in product_id.first..=product_id.last {
            count_of_invalid += check_individual_id(i);
        }
    }

    return count_of_invalid;
}

fn check_individual_id(id: i64) -> i64 {
    let count_of_digits = id.checked_ilog10().unwrap_or(0) + 1;

    if count_of_digits % 2 == 0
    {
        let strval = id.to_string();
        let (lstr, rstr) = strval.split_at((count_of_digits/2) as usize);
        let lnum = lstr.parse::<i64>().unwrap();
        let rnum = rstr.parse::<i64>().unwrap();

        if lnum == rnum {return id};
    }

    return 0;
}

fn process_data(file: String) -> Result<Vec<ProductId>> {
    let ranges_clean = file.replace("\n", "");

    return ranges_clean
        .split(',')
        .map(|r| parse_range(r))
        .collect();
}

fn parse_range(raw: &str) -> Result<ProductId> {
    let (first_raw, second_raw) = raw.split_once('-').unwrap();
    let first = first_raw.parse::<i64>().unwrap();
    let last = second_raw.parse::<i64>().unwrap();

    return Ok(ProductId { first, last });
}