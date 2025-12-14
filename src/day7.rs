use anyhow::Result;

 #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize
}

pub fn run_day_seven(file_as_string: String) -> Result<String> {
    let diagram = process_data(file_as_string);
    let mut split_points = Vec::new();

    trace_beam(&diagram, &mut split_points, Point{y: 1, x: diagram[0].iter().position(|s| *s == 'S').unwrap()});

    let result_value = split_points.len();

    return Ok(format!("Result: {result_value}"));
}

fn trace_beam(diagram: &Vec<Vec<char>>, split_points: &mut Vec<Point>, current_position: Point) {
    if current_position.y == diagram.len() { return }

    for y in current_position.y..diagram.len() {
        let in_path = diagram[y][current_position.x];

        match in_path {
            ' ' => continue,
            '^' => {
                let new_point = Point{y, x:current_position.x};
                if split_points.contains(&new_point) {
                    return;
                }

                split_points.push(new_point);
                trace_beam(diagram, split_points, Point{y: y+1, x: current_position.x - 1});
                trace_beam(diagram, split_points, Point{y: y+1, x: current_position.x + 1});
                return;
            }
            _   => panic!("I dont think we're touring next year fellas")
        }
    }
}

fn process_data(file: String) -> Vec<Vec<char>> {
    return file.replace(".", " ").split('\n').map(|r| r.chars().collect()).collect();
}