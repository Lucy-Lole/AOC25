use anyhow::Result;

type Ingredient = u64;

struct ProductRange {
    first: u64,
    last: u64
}

impl From<(&str, &str)> for ProductRange {
    fn from(value: (&str, &str)) -> Self {
        Self {
            first: value.0.parse::<u64>().unwrap(),
            last: value.1.parse::<u64>().unwrap()
        }
    }
}

pub fn run_day_five(file_as_string: String) -> Result<String> {
    let (fresh, available) = process_data(file_as_string);

    let result_value = find_fresh_ingredients(fresh, available);

    return Ok(format!("Result: {result_value}"));
}

fn find_fresh_ingredients(fresh: Vec<ProductRange>, available: Vec<Ingredient>) -> u64 {
    let mut result = 0;

    for ingredient in available {
        for range in &fresh {
            if ingredient >= range.first && ingredient <=range.last {
                result += 1;
                break;
            }
        }
    }

    return result;
}

fn process_data(file: String) -> (Vec<ProductRange>, Vec<Ingredient>) {
    let (fresh, available): (Vec<&str>, Vec<&str>) = file
        .split('\n')
        .filter(|r| !r.is_empty())
        .partition(|row| row.contains('-'));

    return (to_product_ranges(fresh), to_products(available));
}

fn to_product_ranges(list: Vec<&str>) -> Vec<ProductRange> {
    return list.iter().map(|r| ProductRange::from(r.split_once('-').unwrap())).collect();
}

fn to_products(list: Vec<&str>) -> Vec<Ingredient> {
    return list.iter().map(|r| r.parse::<u64>().unwrap()).collect();
}