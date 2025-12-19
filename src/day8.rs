use std::u64;
use anyhow::Result;

trait DistanceCanBeMeasured {
    fn measure_distance(&self, other: &Self) -> i64;
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
    circuit_id: Option<u64>,
}

 #[derive(Clone, PartialEq, Eq)]
struct Circuit {
    circuit_id: u64,
    points: Vec<usize>
}

struct PointPair {
    p1: usize,
    p2: usize,
    distance: i64
}

impl DistanceCanBeMeasured for Point {
    fn measure_distance(&self, other: &Self) -> i64 {
        return ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)+ (self.z - other.z).pow(2)).isqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z);
    }
}

pub fn run_day_eight(file_as_string: String) -> Result<String> {
    let mut points = process_data(file_as_string);

    let result_value = find_product_of_largest(1000, &mut points);

    return Ok(format!("Result: {result_value}"));
}

fn find_product_of_largest(iterations: u64, points: &mut Vec<Point>) -> u64 {
    let mut next_circuit_id = 0;
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut pairs: Vec<PointPair> = Vec::new();
    let points_length = points.len();

    for point_index in 0..points_length {
        for compare_index in 0..points_length {
            if point_index == compare_index { continue; }
            //else if pairs.iter().any(|a| a.p1 == compare_index && a.p2 == point_index) { continue; };

            let distance = points[point_index].measure_distance(&points[compare_index]);

            pairs.push(PointPair { p1: point_index, p2: compare_index, distance });
        }
    }

    pairs.sort_by(|a, b| a.distance.cmp(&b.distance));

    let mut matches_made = 1;

    for i in 0..pairs.len() {
        if matches_made >= iterations { break; }

        let PointPair { p1, p2, distance: _} = pairs[i as usize];
        match points[p1].circuit_id {
            Some(circuit_id) => {
                match points[p2].circuit_id {
                    Some(other_circuit_id) => {
                        if circuit_id == other_circuit_id { continue; }

                        let other_points = circuits.iter_mut().find(|a| a.circuit_id == other_circuit_id).unwrap().points.clone();

                        circuits.iter_mut().find(|a| a.circuit_id == other_circuit_id).unwrap().points.clear();

                        circuits.iter_mut().find(|a| a.circuit_id == circuit_id).unwrap().points.extend(other_points);

                        points[p2].circuit_id = Some(circuit_id);
                    },
                    None => {
                        points[p2].circuit_id = Some(circuit_id);
                        circuits.iter_mut().find(|a| a.circuit_id == circuit_id).unwrap().points.push(p2);
                    }
                }
            },
            None => {
                match points[p2].circuit_id {
                    Some(other_circuit_id) => {
                        points[p1].circuit_id = Some(other_circuit_id);
                        circuits.iter_mut().find(|a| a.circuit_id == other_circuit_id).unwrap().points.push(p1);
                    },
                    None => {
                        let new_circuit = Circuit{circuit_id: next_circuit_id, points: vec![p1, p2] };
                        points[p2].circuit_id = Some(next_circuit_id);
                        points[p1].circuit_id = Some(next_circuit_id);

                        next_circuit_id += 1;
                        circuits.push(new_circuit);
                    }
                }
            }
        }

        matches_made += 1;
    }


    circuits.sort_by(|a,b| b.points.len().cmp(&a.points.len()));
    return (circuits[0].points.len() * circuits[1].points.len() * circuits[2].points.len()) as u64;
}

fn process_data(file: String) -> Vec<Point> {
    return file
    .split('\n')
    .map(|r| r.split(',').collect())
    .map(|row: Vec<&str>| {
        return Point{x: row[0].parse::<i64>().unwrap(), y: row[1].parse::<i64>().unwrap(),z: row[2].parse::<i64>().unwrap(), circuit_id: Option::None}; 
    })
    .collect();
}