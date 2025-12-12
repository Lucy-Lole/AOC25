use anyhow::Result;

struct Point {
    y: usize,
    x: usize
}

pub fn run_day_four(file_as_string: String) -> Result<String> {
    let grid = process_data(file_as_string);

    let result_value = get_max_rolls(grid);

    return Ok(format!("Result: {result_value}"));
}

fn get_max_rolls(mut grid: Vec<Vec<bool>>) -> u32 {
    let (mut result, mut points) = find_rolls(&grid);

    loop {
        if points.is_empty() { break; }
        for point in points {
            grid[point.y][point.x] = false;
        }

        let (new_result, new_points) = find_rolls(&grid);

        result += new_result;
        points = new_points;
    }

    return result;
}

fn find_rolls(grid: &Vec<Vec<bool>>) -> (u32, Vec<Point>) {
    let mut result = 0;
    let mut points = Vec::new();
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let value = grid[y][x];

            if !value { continue; }

            let mut count = 0;

            if y > 0 && x > 0 &&        grid[y-1][x-1]     { count +=1; }
            if y > 0 &&                 grid[y-1][x]       { count +=1; }
            if y > 0 && x < x_max &&    grid[y-1][x+1]     { count +=1; }
            if x > 0 &&                 grid[y]  [x-1]     { count +=1; }
            if x < x_max &&             grid[y]  [x+1]     { count +=1; }
            if y < y_max && x > 0 &&    grid[y+1][x-1]     { count +=1; }
            if y < y_max &&             grid[y+1][x]       { count +=1; }
            if y < y_max && x < x_max &&grid[y+1][x+1]     { count +=1; }

            if count < 4 {
                points.push(Point{y,x});
                result += 1
            }
        }
    }

    return (result, points);
}

fn process_data(file: String) -> Vec<Vec<bool>> {
    return file
        .split('\n')
        .map(|row| row.chars().map(|c| c == '@').collect())
        .collect();
}