use regex::Regex;
use std::{collections::HashMap, str::FromStr, vec};

type VisitedPathWithCost = (String, Vec<String>, i64);

fn find_best(
    node: String,
    path: &mut Vec<String>,
    nodes: &HashMap<String, Valve>,
    minutes: i64,
    cache: &mut HashMap<VisitedPathWithCost, i64>,
) -> i64 {
    if minutes <= 0 {
        return 0;
    }

    if let Some(&result) = cache.get(&(node.clone(), path.clone(), minutes)) {
        return result;
    }

    let mut best = i64::MIN;
    let current_flow_rate = nodes.get(&node).unwrap().flow_rate;

    // if current node has flow and we didn't open it yet we need to open it, stop to open it (2 mins)
    if current_flow_rate > 0 && !path.contains(&node) {
        for child in &nodes.get(&node).unwrap().tunnels {
            path.push(node.clone());
            let child_pressure = find_best(child.to_string(), path, nodes, minutes - 2, cache);

            let child_best = child_pressure + current_flow_rate * (minutes - 1);
            if best < child_best {
                best = child_best;
            }

            path.pop();
        }
    }

    // for any other node, go to it and then calculate the best value from all its branches
    for child in &nodes.get(&node).unwrap().tunnels {
        let child_pressure = find_best(child.to_string(), path, nodes, minutes - 1, cache);
        if best < child_pressure {
            best = child_pressure;
        }
    }

    // uses cache to avoid revisitting paths` that were already visited
    cache.insert((node, path.clone(), minutes), best);

    best
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
        let mut tunnels: Vec<String> = captures[3].split(", ").map(|x| x.to_string()).collect();
        tunnels.push(name.clone());

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

// fn find_best_for_2_disjoint_sets(
//     all_paths: &HashMap<VisitedPathWithCost, (i64, Vec<String>)>,
// ) -> (i64, Vec<String>, Vec<String>) {
//     let mut best_cost = 0;
//     let mut first_ref_to_path = vec![];
//     let mut second_ref_to_path = vec![];

//     for (_, first) in all_paths {
//         for (_, second) in all_paths {
//             if is_disjoint(first, second) {
//                 let cost_sum = first.0 + second.0;

//                 if cost_sum > best_cost {
//                     best_cost = cost_sum;
//                     first_ref_to_path = first.1.clone();
//                     second_ref_to_path = second.1.clone();
//                 }
//             }
//         }
//     }

//     (best_cost, first_ref_to_path, second_ref_to_path)
// }

// fn is_disjoint(first: &(i64, Vec<String>), second: &(i64, Vec<String>)) -> bool {
//     for node in &first.1 {
//         if second.1.contains(node) {
//             return false;
//         }
//     }

//     true
// }

fn main() {
    // let input = include_str!("../../day16_input");
    let input = include_str!("../../test_inputs/day16_test");
    let valves = parse_valves(input).unwrap();
    let mut cache = HashMap::new();

    // First Phase
    let pressure = find_best("AA".to_string(), &mut vec![], &valves, 30, &mut cache);
    println!("Pressure after 30 minutes: {:?}", pressure);

    // Second Phase Idea:
    // Instead of finding the BEST one from the find_best solution I can
    // Get all of the paths, then find the disjoint ones (I won't ever open the same valve twice)
    // If I sum the pressure released sets I can then find the pressure for the second actor
    // (I can even do it for N elephants possibly?)

    // Reviewing this now before christmas:
    // I cannot actually do it the way it is now because I am doing a top-down approach to finding the best value.
    // If I change it to do a bottom-up approach I can actually do it because I will have the partial value
    // calculated for any given node.
    // The issue with the previous solution is that I cannot use the cache to do it since it stores the BEST path value
    // UNDER the given node, not the path itself for the spent time to open it (even though it shows the correct solution)
    // as one of the possible answers, I would need to -rebuild- the cost minute by minute to find the actual value
    // and THEN compare it to the other elements, it is easier to rewrite everything as bottom-up instead but hey
    // 3 hours for christmas I am not doing that rn.

    let mut cache = HashMap::new();

    find_best("AA".to_string(), &mut vec![], &valves, 26, &mut cache);

    // println!(
    //     "{:?}",
    //     cache
    //         .clone()
    //         .into_iter()
    //         .filter(|(thing, _)| thing.1 == vec!["DD", "HH", "EE"] && thing.0 == "EE")
    //         .map(|x| {
    //             println!("{:?}", x);
    //             x
    //         })
    //         // .collect::<HashMap<VisitedPathWithCost, i64>>()
    //         .max_by(|((_, _, min1), _), ((_, _, min2), _)| min1.cmp(&min2))
    //         .unwrap()
    // );

    // println!(
    //     "{:?}",
    //     cache
    //         .clone()
    //         .into_iter()
    //         .filter(|(thing, _)| thing.1 == vec!["JJ", "BB", "CC"] && thing.0 == "CC")
    //         // .collect::<HashMap<VisitedPathWithCost, i64>>()
    //         .max_by(|((_, _, min1), _), ((_, _, min2), _)| min1.cmp(&min2))
    //         .unwrap()
    // );

    // println!("Pressure after 26 minutes: {:?}", pressure);

    // let best_with_elephant = find_best_for_2_disjoint_sets(&cache);
    // println!("Best with 2: {:?}", best_with_elephant);
}
