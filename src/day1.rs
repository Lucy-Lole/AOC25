use anyhow::Result;

struct Direction {
    is_right: bool,
    value: i32
}

pub fn run_day_one (file: String) -> Result<String> {
    let lines = file.split('\n');

    let directions_data: Result<Vec<Direction>, _> = lines
        .filter(|l| l.len() > 1)
        .map(|l| parse_direction(l))
        .collect();

    let directions = match directions_data {
        Ok(directions) => directions,
        Err(_) => panic!("Failed to parse data, yes I know I shouldnt use panic stop reading this.")
    };

    let mut current_position: i32 = 50;
    let mut counter = 0;

    for dir in directions {
        let (new_pos, clicks) = perform_rotation(current_position, &dir);

        current_position = new_pos;
        counter += clicks;
    }

    return Ok(format!("Result: done with {}", counter));
}

fn perform_rotation(current_position: i32, movement: &Direction) -> (i32, i32) {
    let new_absolute =
        if movement.is_right {current_position + movement.value}
        else                 {current_position - movement.value};

    let new_final =
        if movement.is_right             { (new_absolute) % 100 }
        else { new_absolute.rem_euclid(100) };

    let delta_to_click = 
        if movement.is_right {100 - current_position}
        else                 {current_position};

    let clicks =
        if movement.value.abs() >= delta_to_click {
            let overclick = (movement.value.abs() - delta_to_click) / 100;
            if delta_to_click != 0
                { 1 + overclick }
            else
                { overclick }
        }
        else {0};

    return (new_final, clicks);
}

type DirectionResult = Result<Direction, &'static str>;

fn parse_direction(line: &str) -> DirectionResult {
    let ( polarity_str, value_str ) = line.split_at(1);

    let polarity = match polarity_str {
        "L" => false,
        "R" => true,
        _ => return Err("Error parsing direction letter")
    };

    let value = match value_str.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err("Error parsing direction int")
    };

    return Ok(Direction { is_right: (polarity), value: (value) })
}