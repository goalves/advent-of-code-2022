use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day15");
    // let input = include_str!("../../inputs/test/day15");

    let sensors: Vec<InputSensor> = input
        .trim()
        .split("\n")
        .into_iter()
        .map_into()
        .collect::<Vec<InputSensor>>();

    let beacon_positions = sensors
        .iter()
        .map(|s| s.closest_beacon.clone())
        .collect::<Vec<Point>>();

    phase_1(&sensors, &beacon_positions, 2000000);
    phase_2(&sensors, &beacon_positions, 4000000);
}

fn phase_1(sensors: &Vec<InputSensor>, beacon_positions: &Vec<Point>, line_to_look: i64) {
    let start = max_possible_x_in_direction(&sensors, -1);
    let finish = max_possible_x_in_direction(&sensors, 1);

    let mut current = start;
    let mut number_of_impossible_beacon_positions = 0;
    loop {
        if current > finish {
            break;
        }

        let current_point = Point {
            x: current,
            y: line_to_look,
        };

        if !point_can_have_a_beacon(&current_point, &sensors)
            && !beacon_positions.contains(&current_point)
        {
            number_of_impossible_beacon_positions += 1;
        };

        current += 1;
    }

    println!(
        "Phase 1: Impossible beacons on {}th line: {}",
        line_to_look, number_of_impossible_beacon_positions
    );
}

fn max_possible_x_in_direction(sensors: &Vec<InputSensor>, direction: i64) -> i64 {
    sensors
        .iter()
        .map(|sensor| max_point_in_line_direction(sensor, direction))
        .max_by(|x, y| {
            if direction < 0 {
                y.x.cmp(&x.x)
            } else {
                x.x.cmp(&y.x)
            }
        })
        .unwrap()
        .x
}

fn max_point_in_line_direction(sensor: &InputSensor, direction: i64) -> Point {
    Point {
        x: sensor.position.x + (direction * sensor.distance as i64) as i64,
        ..sensor.position
    }
}

fn point_can_have_a_beacon(point: &Point, matrix: &Vec<InputSensor>) -> bool {
    for sensor in matrix {
        let point_dist = m_dist(point, &sensor.position);
        if point_dist <= sensor.distance {
            return false;
        }
    }

    true
}

fn m_dist(a: &Point, b: &Point) -> u64 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u64
}

fn phase_2(sensors: &Vec<InputSensor>, beacon_positions: &Vec<Point>, max: i64) {
    let mut edge_points = Vec::new();

    for sensor in sensors {
        let sensor_edges = sensor_edge_points(sensor);
        edge_points.push(sensor_edges)
    }

    for point in edge_points
        .iter()
        .unique()
        .flatten()
        .filter(|point| point.x >= 0 && point.x <= max && point.y >= 0 && point.y <= max)
    {
        if point_can_have_a_beacon(&point, &sensors) && !beacon_positions.contains(&point) {
            println!(
                "Phase 2: {:?} with frequency: {}",
                point,
                point.x * 4000000 + point.y
            );

            return;
        }
    }

    panic!("shouldn't ever not find a point :)")
}

fn sensor_edge_points(sensor: &InputSensor) -> Vec<Point> {
    let mut edges = Vec::new();
    let dist = sensor.distance as i64;

    let mut x_diff = -dist - 1;
    let mut y_diff = 0;

    loop {
        if x_diff > 0 {
            break;
        }

        edges.push(Point {
            x: sensor.position.x + x_diff,
            y: sensor.position.y + y_diff,
        });
        edges.push(Point {
            x: sensor.position.x - x_diff,
            y: sensor.position.y + y_diff,
        });

        edges.push(Point {
            x: sensor.position.x + x_diff,
            y: sensor.position.y - y_diff,
        });
        edges.push(Point {
            x: sensor.position.x - x_diff,
            y: sensor.position.y - y_diff,
        });

        y_diff += 1;
        x_diff += 1;
    }

    edges
}

#[derive(Debug, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct InputSensor {
    position: Point,
    closest_beacon: Point,
    distance: u64,
}

impl From<&str> for InputSensor {
    fn from(value: &str) -> Self {
        let re =
            Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(value).unwrap();
        let position = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };

        let closest_beacon = Point {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap(),
        };

        let distance = m_dist(&position, &closest_beacon);

        InputSensor {
            position,
            closest_beacon,
            distance,
        }
    }
}
