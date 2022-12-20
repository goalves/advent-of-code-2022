use std::fmt::Display;

use itertools::Itertools;
use regex::Regex;

fn main() {
    // let input = include_str!("../../day15_input");
    let input = include_str!("../../test_inputs/day15_test");

    let sensors: Vec<InputSensor> = input
        .trim()
        .split("\n")
        .into_iter()
        .map_into()
        .collect::<Vec<InputSensor>>();

    let mut points_without_beacon = Vec::new();

    for sensor in &sensors {
        points_without_beacon.push(points_without_a_beacon(&sensor, 10));
    }

    for sensor in &sensors {
        points_without_beacon.push(Points(vec![sensor.position.clone()]));
    }

    println!(
        "{}",
        points_without_beacon
            .clone()
            .into_iter()
            .reduce(|mut acc, el| {
                for i in el.0.clone() {
                    acc.0.push(i);
                }
                acc
            })
            .unwrap()
    );

    let beacons = sensors
        .iter()
        .map(|x| x.closest_beacon.clone())
        .collect::<Vec<Point>>();

    let count = points_without_beacon
        .into_iter()
        .flat_map(|x| x.0)
        .filter(|x| x.y == 10 && !beacons.contains(x))
        .unique()
        .map(|x| {
            println!("oi meu x: {:?}", x);
            x
        })
        .count();

    println!("Count at line 10: {}", count);
}

fn points_without_a_beacon(sensor: &InputSensor, line: i64) -> Points {
    let m_dist_without_beacons = m_dist(&sensor.position, &sensor.closest_beacon);
    let mut imposible_beacon_positions = vec![sensor.closest_beacon.clone()];
    let mut x_dif: i64 = 0 - m_dist_without_beacons as i64;

    loop {
        let mut y_dif: i64 = (m_dist_without_beacons as i64 - x_dif.abs()).abs();
        loop {
            if sensor.position.y + y_dif == line || sensor.position.y - y_dif == line {
                imposible_beacon_positions.push(Point {
                    x: sensor.position.x + x_dif,
                    y: sensor.position.y + y_dif,
                    t: PointType::Nothing,
                });
                imposible_beacon_positions.push(Point {
                    x: sensor.position.x + x_dif,
                    y: sensor.position.y - y_dif,
                    t: PointType::Nothing,
                });
            }

            if y_dif == 0 {
                break;
            }

            y_dif -= 1;
        }

        x_dif += 1;
        if x_dif > m_dist_without_beacons as i64 {
            break;
        }
    }

    Points(imposible_beacon_positions)
}

#[derive(Debug, Clone)]
struct Points(Vec<Point>);

impl From<Points> for Vec<Point> {
    fn from(value: Points) -> Self {
        value.0
    }
}

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();
        let mut x_start = i64::MAX;
        let mut x_end = i64::MIN;
        let mut y_start = i64::MAX;
        let mut y_end = i64::MIN;

        for element in &self.0 {
            if element.x > x_end {
                x_end = element.x;
            }
            if element.x < x_start {
                x_start = element.x;
            }

            if element.y > y_end {
                y_end = element.y;
            }
            if element.y < y_start {
                y_start = element.y;
            }
        }

        let base_info = format!(
            "\nStarting on ({}, {}), finishing on: ({}, {})\n",
            x_start, y_start, x_end, y_end
        );
        output_string.push_str(&base_info.to_owned());

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                if self.0.contains(&Point {
                    y,
                    x,
                    t: PointType::Sensor,
                }) {
                    output_string.push_str("🖥️")
                } else if self.0.contains(&Point {
                    y,
                    x,
                    t: PointType::Beacon,
                }) {
                    output_string.push_str("❇️")
                } else if self.0.contains(&Point {
                    y,
                    x,
                    t: PointType::Nothing,
                }) {
                    output_string.push_str("❌")
                } else {
                    output_string.push_str("🪨")
                }
            }
            output_string.push_str("\n");
        }

        write!(f, "{}", output_string)
    }
}

fn m_dist(a: &Point, b: &Point) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

#[derive(Debug, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
    t: PointType,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum PointType {
    Beacon,
    Sensor,
    Nothing,
}

#[derive(Debug)]
struct InputSensor {
    position: Point,
    closest_beacon: Point,
}

impl From<&str> for InputSensor {
    fn from(value: &str) -> Self {
        let re =
            Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(value).unwrap();
        InputSensor {
            position: Point {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                t: PointType::Sensor,
            },
            closest_beacon: Point {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
                t: PointType::Beacon,
            },
        }
    }
}
