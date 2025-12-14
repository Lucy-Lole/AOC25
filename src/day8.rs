use std::u64;

use anyhow::Result;

trait DistanceCanBeMeasured {
    fn measure_distance(&self, other: &Self) -> u64;
}

struct Point<'a>{
    x: u64,
    y: u64,
    z: u64,
    circuit: Option<&'a mut Vec<&'a mut Point<'a>>>
}

impl DistanceCanBeMeasured for Point<'_> {
    fn measure_distance(&self, other: &Self) -> u64 {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z);
    }
}

impl PartialEq for Point<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z);
    }
}

pub fn run_day_eight(file_as_string: String) -> Result<String> {
    let points = process_data(file_as_string);

    let result_value = points.len();

    return Ok(format!("Result: {result_value}"));
}

fn find_product_of_largest(iterations: u64, points: &Vec<Point>) -> u64 {
    let mut circuits: Vec<&Vec<&mut Point>> = Vec::new();
    let mut connections_made = 0;

    for i in 0..points.len() {
        if connections_made > iterations { break; }

        let ref mut point = points[0];
        let ref mut lowest_distance_point = Point{x:u64::MAX, y:u64::MAX, z:u64::MAX, circuit: Option::None};
        let mut lowest_distance = point.measure_distance(&lowest_distance_point);

        for ref mut other in points {
            if point == *other { continue; }

            let distance = point.measure_distance(other);

            if distance < lowest_distance {
                lowest_distance_point = other;
                lowest_distance = distance;
            }
        }

        match point.circuit {
            Some(circuit) => {
                match lowest_distance_point.circuit {
                    Some(other_circuit) => {

                    },
                    None => todo!(),
                }
            },
            None => {
                match lowest_distance_point.circuit {
                    Some(other_circuit) => {
                        point.circuit = Some(other_circuit);
                        other_circuit.push(point);
                    },
                    None => {
                        let mut new_circuit = Vec::new();
                        new_circuit.push(point);
                        new_circuit.push(lowest_distance_point);
                        circuits.push(&new_circuit);
                        point.circuit = Some(&mut new_circuit);
                        lowest_distance_point.circuit = Some(&mut new_circuit);
                    }
                }
            }
        }
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    return (circuits[0].len() * circuits[1].len() * circuits[2].len()) as u64;
}

fn process_data(file: String) -> Vec<Point<'static>> {
    return file
    .split('\n')
    .map(|r| r.split(',').collect())
    .map(|row: Vec<&str>| Point{x: row[0].parse::<u64>().unwrap(), y: row[1].parse::<u64>().unwrap(),z: row[2].parse::<u64>().unwrap(), circuit: Option::None })
    .collect();
}