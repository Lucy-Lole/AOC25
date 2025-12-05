use std::{fs};

struct Direction {
    is_right: bool,
    value: i32
}

fn main() -> Result<(), &'static str> {

    let file = match fs::read_to_string("./src/inputs/day1.txt") {
        Ok(f) => f,
        Err(e) => panic!("Problem reading input file: {e:?}"),
    };

    let lines = file.split('\n');

    let directions_data: Result<Vec<Direction>, _> = lines
        .filter(|l| l.len() > 1)
        .map(|l| parse_direction(l))
        .collect();

    let directions = match directions_data {
        Ok(directions) => directions,
        Err(e) => return Err(e)
    };

    let mut current_position: i32 = 50;
    let mut counter = 0;

    for dir in directions {
        let (new_pos, clicks) = perform_rotation(current_position, &dir);

        current_position = new_pos;
        counter += clicks;
    }

    println!("Done with {counter}");
    return Ok(());
}

fn perform_rotation(current_position: i32, movement: &Direction) -> (i32, i32) {
    let new_absolute =
        if movement.is_right {current_position + movement.value} 
        else {current_position - movement.value};

    let clicks =
        if movement.is_right {(new_absolute) / 100} 
        else if current_position != 0 || movement.value > 99 {(-new_absolute + 100) / 100}
        else {0};

    let new_final =
        if movement.is_right {(new_absolute) % 100 }
        else { (100 + new_absolute) % 100 };

    return (new_final, clicks);
}

type DirectionResult = Result<Direction, &'static str>;

fn parse_direction(line: &str) -> DirectionResult {
    let ( polarity_str, value_str ) = line.split_at(1);

    let polarity = match polarity_str {
        "L" => false,
        "R" => true,
        _ => return Err("AA")
    };

    let value = match value_str.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err("Error parsing direction int")
    };

    return Ok(Direction { is_right: (polarity), value: (value) })
}
