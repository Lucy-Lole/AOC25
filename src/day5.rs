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
    let (mut fresh, available) = process_data(file_as_string);

    let result_value = find_fresh_ingredients(&fresh, &available);
    let result_2 = find_max_fresh(&mut fresh);

    return Ok(format!("Result P1: {result_value}, Result P2: {result_2}"));
}

fn find_max_fresh(fresh: &mut Vec<ProductRange>) -> u64 {
    let new_fresh = find_combos(fresh);

    return  new_fresh.iter().map(|f| f.last - f.first + 1).sum();
}

fn find_combos(ranges: &mut Vec<ProductRange>) -> &mut Vec<ProductRange> {

    for i_range in 0..ranges.len() {
        for i_other in 0..ranges.len() {
            if i_range == i_other { continue; }

            let range = &ranges[i_range];
            let other = &ranges[i_other];

            if (range.first >= other.first && range.first <= other.last) 
            || (range.last >= other.first &&  range.last <= other.last) {
                let first = range.first.min(other.first);
                let last = range.last.max(other.last);

                let new_range = ProductRange{first, last};

                ranges.remove(i_range);
                if i_range < i_other  { ranges.remove(i_other - 1); }
                else                  { ranges.remove(i_other); }
                ranges.push(new_range);

                return find_combos(ranges);
            }
        }
    }

    return ranges;
}

fn find_fresh_ingredients(fresh: &Vec<ProductRange>, available: &Vec<Ingredient>) -> u64 {
    let mut result = 0;

    for ingredient in available {
        for range in fresh {
            if *ingredient >= range.first && *ingredient <=range.last {
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