use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

impl Point3D {
    fn from_str(s: &str) -> Point3D {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            panic!("Invalid point string: {}", s);
        }
        let x = parts[0].parse::<u32>().expect("should have");
        let y = parts[1].parse::<u32>().expect("should have");
        let z = parts[2].parse::<u32>().expect("should have");
        Point3D { x, y, z }
    }

    fn distance(self: &Self, other: &Point3D) -> f64 {
        let dx = (self.x as i32 - other.x as i32) as f64;
        let dy = (self.y as i32 - other.y as i32) as f64;
        let dz = (self.z as i32 - other.z as i32) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct PairDistance {
    point1: Point3D,
    point2: Point3D,
    distance: f64,
}

impl PairDistance {
    fn from(p1: Point3D, p2: Point3D) -> PairDistance {
        let distance = p1.distance(&p2);
        PairDistance {
            point1: p1,
            point2: p2,
            distance,
        }
    }
}

impl PartialEq for PairDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for PairDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse order for max-heap behavior
        other.distance.partial_cmp(&self.distance)
    }
}

impl Eq for PairDistance {}

impl Ord for PairDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order for max-heap behavior
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

pub fn part1(input: &str, max_connections: usize) -> usize {
    let mut points = Vec::new();
    for line in input.lines() {
        points.push(Point3D::from_str(line));
    }

    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            heap.push(PairDistance::from(points[i].clone(), points[j].clone()));
        }
    }

    let mut circuits = Vec::<HashSet<Point3D>>::new();
    let mut visited = HashSet::<Point3D>::new();
    let mut connections = 0;
    while let Some(pair) = heap.pop() && connections < max_connections {
        connections += 1;
        let (p1, p2) = (&pair.point1, &pair.point2);

        // Both points have been seen already
        if visited.contains(p1) && visited.contains(p2) {
            // check which circuits they are in
            let p1_circuit = circuits.iter().position(|x| x.contains(p1)).expect("must be in a circuit already");
            let p2_circuit = circuits.iter().position(|x| x.contains(p2)).expect("must be in a circuit already");

            if p1_circuit < p2_circuit {
                let mut to_merge = circuits.swap_remove(p2_circuit);
                for p in to_merge.drain() {
                    circuits[p1_circuit].insert(p.clone());
                }
            } else if p2_circuit < p1_circuit {
                let mut to_merge = circuits.swap_remove(p1_circuit);
                for p in to_merge.drain() {
                    circuits[p2_circuit].insert(p.clone());
                }
            }
            continue;
        }

        let mut circuit_idx = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(p1) || circuit.contains(p2) {
                circuit_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = circuit_idx {
            circuits[idx].insert(p1.clone());
            circuits[idx].insert(p2.clone());
        } else {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(p1.clone());
            new_circuit.insert(p2.clone());
            circuits.push(new_circuit);
        }

        // Mark points as visited
        visited.insert(p1.clone());
        visited.insert(p2.clone());
    }

    // Find the 3 largest circuits
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut total = 1;
    for i in 0..(3.min(circuits.len())) {
        total *= circuits[i].len();
    }
    println!("Day 8, Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> u64 {
    let mut points = Vec::new();
    for line in input.lines() {
        points.push(Point3D::from_str(line));
    }

    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            heap.push(PairDistance::from(points[i].clone(), points[j].clone()));
        }
    }

    let mut circuits = Vec::<HashSet<Point3D>>::new();
    let mut visited = HashSet::<Point3D>::new();
    let mut last_connection = None;
    while let Some(pair) = heap.pop() {
        let (p1, p2) = (&pair.point1, &pair.point2);

        // Both points have been seen already
        if visited.contains(p1) && visited.contains(p2) {
            // check which circuits they are in
            let p1_circuit = circuits.iter().position(|x| x.contains(p1)).expect("must be in a circuit already");
            let p2_circuit = circuits.iter().position(|x| x.contains(p2)).expect("must be in a circuit already");

            if p1_circuit < p2_circuit {
                let mut to_merge = circuits.swap_remove(p2_circuit);
                for p in to_merge.drain() {
                    circuits[p1_circuit].insert(p.clone());
                }
                last_connection = Some(pair);
            } else if p2_circuit < p1_circuit {
                let mut to_merge = circuits.swap_remove(p1_circuit);
                for p in to_merge.drain() {
                    circuits[p2_circuit].insert(p.clone());
                }
                last_connection = Some(pair);
            }
            continue;
        }

        let mut circuit_idx = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(p1) || circuit.contains(p2) {
                circuit_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = circuit_idx {
            circuits[idx].insert(p1.clone());
            circuits[idx].insert(p2.clone());
        } else {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(p1.clone());
            new_circuit.insert(p2.clone());
            circuits.push(new_circuit);
        }

        // Mark points as visited
        visited.insert(p1.clone());
        visited.insert(p2.clone());
        last_connection = Some(pair);

        if circuits[0].len() == points.len() {
            break;
        }
    }

    let mut total = 0;
    if let Some(pair) = last_connection {
        total = (pair.point1.x as u64 * pair.point2.x as u64) as u64;
    }
    println!("Day 8, Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"#;
        assert_eq!(part1(input, 10), 40);
    }

    #[test]
    fn test_part2() {
        let input = r#"162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"#;
        assert_eq!(part2(input), 25272);
    }
}
