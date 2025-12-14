use anyhow::Result;

 #[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    y: usize,
    x: usize
}

 #[derive(Copy, Clone)]
struct QuantumPoint {
    y: usize,
    x: usize,
    universes: u64
}

impl PartialEq for QuantumPoint {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

pub fn run_day_seven(file_as_string: String) -> Result<String> {
    let diagram = process_data(file_as_string);
    let mut split_points = Vec::new();
    let mut quantum_split_points = Vec::new();

    trace_beam(&diagram, &mut split_points, Point{y: 1, x: diagram[0].iter().position(|s| *s == 'S').unwrap()});
    let result_value_1 = split_points.len();

    let result_value_2 = trace_beam_quantum(&diagram, &mut quantum_split_points, Point{y: 1, x: diagram[0].iter().position(|s| *s == 'S').unwrap()});

    return Ok(format!("Result P1: {result_value_1}, Result P2: {result_value_2}"));
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
            _   => panic!("I'm not real sure we're gonna be touring next year fellas")
        }
    }
}

fn trace_beam_quantum(diagram: &Vec<Vec<char>>, split_points: &mut Vec<QuantumPoint>, current_position: Point) -> u64 {
    for y in current_position.y..diagram.len() {
        let in_path = diagram[y][current_position.x];

        match in_path {
            ' ' => continue,
            '^' => {
                let mut new_point = QuantumPoint{y, x:current_position.x, universes: 0};

                if split_points.contains(&new_point) {
                    return split_points.iter().find(|&a| a.eq(&new_point)).unwrap().universes;
                }

                let universes_from_here_on_out =
                    trace_beam_quantum(diagram, split_points, Point{y: y+1, x: current_position.x - 1}) +
                    trace_beam_quantum(diagram, split_points, Point{y: y+1, x: current_position.x + 1});

                new_point.universes = universes_from_here_on_out;
                split_points.push(new_point);
                return universes_from_here_on_out;
            }
            _   => panic!("I'm not real sure we're gonna be touring next year fellas")
        }
    }

    return 1;
}

fn process_data(file: String) -> Vec<Vec<char>> {
    return file.replace(".", " ").split('\n').map(|r| r.chars().collect()).collect();
}