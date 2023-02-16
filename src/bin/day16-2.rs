use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::BufRead,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day16.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();

    let mut valve_flow_rate = HashMap::<String, usize>::new();
    let mut valve_edges = HashMap::<String, Vec<String>>::new();
    let mut flow_positive_valves = HashSet::<String>::new();

    for line_result in lines {
        let line = line_result.ok().unwrap();
        let valve_name_idx = line.find("Valve ").unwrap() + 6;
        let valve_name = &line[valve_name_idx..valve_name_idx + 2];
        let flow_rate_idx = line.find("=").unwrap() + 1;
        let flow_rate_end_idx = line.find(";").unwrap();
        let flow_rate = *&line[flow_rate_idx..flow_rate_end_idx]
            .parse::<usize>()
            .ok()
            .unwrap();
        let to_valves = line[line.rfind("valve").unwrap() + 7..line.len()]
            .split(",")
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<String>>();
        valve_flow_rate.insert(valve_name.to_owned(), flow_rate);
        if flow_rate > 0 || valve_name == "AA" {
            flow_positive_valves.insert(valve_name.to_owned());
        }
        valve_edges.insert(valve_name.to_owned(), to_valves);
    }

    let mut weighted_graph = HashMap::<String, HashMap<String, i32>>::new();
    for valve in &flow_positive_valves {
        // get shortest distance to all other nodes with flow rate
        let mut queue = BinaryHeap::new();
        queue.push((0, valve));
        let mut visited = HashSet::new();
        weighted_graph.insert(valve.to_owned(), HashMap::<String, i32>::new());
        while queue.len() > 0 {
            let pair = queue.pop().unwrap();
            let distance = pair.0;
            let curr_valve = pair.1;
            visited.insert(curr_valve.clone());
            for edge in valve_edges.get(curr_valve).unwrap() {
                if !visited.contains(edge) {
                    queue.push((distance - 1, edge));
                    if flow_positive_valves.contains(edge) {
                        let map = weighted_graph.get_mut(valve).unwrap();
                        if !map.contains_key(edge) {
                            map.insert(edge.to_owned(), -1 * (distance - 1));
                        }
                    }
                }
            }
        }
    }

    let mut queue = BinaryHeap::new();
    let mut flow_map = HashMap::<Vec<String>, i32>::new();
    queue.push((26, 0, "AA", Vec::<String>::new()));
    while let Some(el) = queue.pop() {
        let minutes_remaining = el.0;
        if minutes_remaining <= 0 {
            break;
        }
        let flow_rate = el.1;
        let valve = el.2;
        let visited = el.3;
        for edge in weighted_graph.get(valve).unwrap() {
            let edge_name = edge.0;
            // ignore starting node
            if edge_name == "AA" {
                continue;
            }
            // dont go to edge if already visited it
            if visited.contains(edge_name) {
                continue;
            }
            let edge_flow_rate = *valve_flow_rate.get(edge_name).unwrap();
            let distance = *edge.1;
            let flow_activation_cost = 1;
            let new_minutes_remaining = minutes_remaining - flow_activation_cost - distance;
            if new_minutes_remaining < 0 {
                if !flow_map.contains_key(&visited) {
                    flow_map.insert(visited.clone(), flow_rate);
                } else {
                    flow_map.insert(
                        visited.clone(),
                        *flow_map.get(&visited).unwrap().max(&flow_rate),
                    );
                }
            }
            let new_flow_rate = flow_rate + (new_minutes_remaining * edge_flow_rate as i32);
            let mut new_visited = visited.clone();
            new_visited.push(edge_name.to_string());
            queue.push((new_minutes_remaining, new_flow_rate, edge_name, new_visited));
        }
    }

    let mut max_flow = 0;
    let total = flow_map.len();
    let mut keys: Vec<(i32, Vec<String>)> = vec![];
    for pair in flow_map {
        keys.push((pair.1, pair.0));
    }
    keys.sort();
    keys.reverse();
    for first_idx in 0..total {
        let first = &keys[first_idx];
        let first_score = first.0;
        let first_path: HashSet<String> = HashSet::from_iter(first.1.clone());
        for second_idx in first_idx + 1..total {
            let second = &keys[second_idx];
            let second_score = second.0;
            let second_path = HashSet::from_iter(second.1.clone());
            if first_score + second_score < max_flow {
                break;
            }
            if first_path.is_disjoint(&second_path) {
                max_flow = max_flow.max(first_score + second_score);
            }
        }
    }
    println!("max flow {}", max_flow);
    Ok(())
}
