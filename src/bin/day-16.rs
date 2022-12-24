use regex::Regex;
use std::{collections::HashMap, str::FromStr};

type VisitedPathWithCost = (String, Vec<String>, i64);

fn find_best(
    node: String,
    path: &mut Vec<String>,
    nodes: &HashMap<String, Valve>,
    minutes: i64,
    cache: &mut HashMap<VisitedPathWithCost, i64>,
) -> (i64, Vec<String>) {
    if minutes <= 0 {
        return (0, path.clone());
    }

    if let Some(&cached_cost) = cache.get(&(node.clone(), path.clone(), minutes)) {
        return (cached_cost, path.clone());
    }

    let mut best = i64::MIN;
    let mut best_path = path.clone();
    let current_flow_rate = nodes.get(&node).unwrap().flow_rate;

    // if current node has flow, always stop to open it and we didn't open it yet we need to open it (2 mins)
    if current_flow_rate > 0 && !path.contains(&node) {
        for child in &nodes.get(&node).unwrap().tunnels {
            path.push(node.clone());
            let (child_pressure, child_best_path) =
                find_best(child.to_string(), path, nodes, minutes - 2, cache);

            let child_best = child_pressure + current_flow_rate * (minutes - 1);
            if best < child_best {
                best_path = child_best_path;
                best = child_best;
            }

            path.pop();
        }
    }

    // for any other node, go to it and then calculate the best value from all its branches
    for child in &nodes.get(&node).unwrap().tunnels {
        let (child_pressure, child_best_path) =
            find_best(child.to_string(), path, nodes, minutes - 1, cache);
        if best < child_pressure {
            best = child_pressure;
            best_path = child_best_path;
        }
    }

    // uses cache to avoid revisitting paths that were already visited
    cache.insert((node, path.clone(), minutes), best);

    (best, best_path)
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i64,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
                .unwrap();

        let captures = re.captures(s).ok_or("invalid valve specification")?;
        let name = captures[1].to_string();
        let flow_rate = captures[2].parse().map_err(|_| "invalid flow rate")?;
        let tunnels = captures[3].split(", ").map(|x| x.to_string()).collect();

        Ok(Valve {
            name,
            flow_rate,
            tunnels,
        })
    }
}

fn parse_valves(input: &str) -> Result<HashMap<String, Valve>, &'static str> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let valve: Valve = line.parse()?;
        valves.insert(valve.name.clone(), valve);
    }

    Ok(valves)
}

fn main() {
    // let input = include_str!("../../day16_input");
    let input = include_str!("../../test_inputs/day16_test");
    let valves = parse_valves(input).unwrap();

    // First Phase
    let pressure = find_best(
        "AA".to_string(),
        &mut vec![],
        &valves,
        30,
        &mut HashMap::new(),
    );
    println!("Pressure after 30 minutes: {:?}", pressure);

    // Second Phase Idea:
    // Instead of finding the BEST one from the find_best solution I can
    // Get all of the paths, then find the disjoint ones (I won't ever open the same valve twice)
    // If I sum the pressure released sets I can then find the pressure for the second actor
    // (I can even do it for N elephants possibly?)
}
