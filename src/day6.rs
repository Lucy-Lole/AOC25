use anyhow::Result;

 #[derive(Copy, Clone)]
enum Operation {
    Add,
    Multiply
}

struct Equation {
    values: Vec<u64>,
    operation: Operation
}

trait Performable {
    fn perform(&self) -> u64;
}

impl Performable for Equation {
    fn perform(&self) -> u64 {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product(),
        }
    }
}

pub fn run_day_six(file_as_string: String) -> Result<String> {
    let equations_part_one = process_data_part_one(&file_as_string);
    let equations_part_two = process_data_part_two(&file_as_string);

    let result_value_part1 = execute_equations(&equations_part_one);
    let result_value_part2 = execute_equations(&equations_part_two);

    return Ok(format!("Result P1: {result_value_part1} | Result P2: {result_value_part2}"));
}

fn execute_equations(equations: &Vec<Equation>) -> u64 {
    return equations.iter().map(|e| e.perform()).sum();
}

fn process_data_part_two(file: &String) -> Vec<Equation> {
    let (nums, ops): (Vec<&str>, Vec<&str>) = file.split('\n').partition(|l| !l.contains('*'));
    let mut nums_iters: Vec<std::str::Chars<'_>> = nums.iter().map(|r| r.chars().into_iter()).collect();

    let ops_parsed: Vec<Operation> = ops[0].replace(" ", "").chars().map(|c| if c == '*' {Operation::Multiply} else {Operation::Add}).collect();

    let mut equations = Vec::new();

    for op in ops_parsed {
        let mut new_eq = Equation{values: Vec::new(), operation: op};

        loop {
            let mut new_val = Vec::new();

            for row in &mut nums_iters {
                let opt = row.next();

                match opt {
                    Some(c) => new_val.push(c.to_owned()),
                    None    => continue
                }
            }

            if new_val.iter().all(|c| c.is_whitespace()) {
                break;
            }
            else {
                let as_string: String = new_val.into_iter().collect();
                new_eq.values.push(as_string.trim().trim_start().parse::<u64>().unwrap());
            }
        }

        equations.push(new_eq);
    }

    return equations;
}

fn process_data_part_one(file: &String) -> Vec<Equation> {
    let (nums, ops): (Vec<&str>, Vec<&str>) = file.split('\n').partition(|l| !l.contains('*'));

    let nums_parsed: Vec<Vec<u64>> = nums
        .iter()
        .map(|r| r.split(' ').filter(|c| !c.is_empty()).map(|str| str.parse::<u64>().unwrap()).collect())
        .collect();

    let ops_parsed: Vec<Operation> = ops[0].replace(" ", "").chars().map(|c| if c == '*' {Operation::Multiply} else {Operation::Add}).collect();

    let line_length = nums_parsed[0].len();

    let mut equations = Vec::new();

    for i in 0..line_length {
        let mut values = Vec::new();

        for j in 0..nums_parsed.len() {
            values.push(nums_parsed[j][i])
        }

        equations.push(Equation{ values, operation: ops_parsed[i]});
    };

    return equations;
}