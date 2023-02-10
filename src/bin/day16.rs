use core::num;
use std::{
    borrow::BorrowMut,
    collections::{BinaryHeap, HashMap, HashSet},
    io::BufRead,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day16.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut valve_flow_rate = HashMap::<String, usize>::new();
    let mut valve_edges = HashMap::<String, Vec<String>>::new();
    let mut num_non_zero_flow = 0;
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
        valve_edges.insert(valve_name.to_owned(), to_valves);
        if flow_rate > 0 {
            num_non_zero_flow += 1;
        }
    }

    let mut opened = HashSet::<String>::new();
    let mut minutes_elapsed = 0;
    let current_pos = "AA";
    while minutes_elapsed < 30 {
        let mut max_flow = (std::i32::MIN, vec![]);
        let mut visited = HashSet::<String>::new();
        visited.insert(current_pos.to_string());
        let edges = valve_edges.get(current_pos).unwrap();
        let mut queue = BinaryHeap::new();
        for edge in edges {
            queue.push((-1, vec![edge]));
        }
        let minutes_remaining = 30 - minutes_elapsed;
        while queue.len() > 0 {
            let item = queue.pop().unwrap();
            let distance: i32 = item.0;
            let path = item.1;
            let last_elem = path[path.len() - 1];
            visited.insert(last_elem.to_string());
            let flow_calc = (minutes_remaining - distance.abs() - 1)
                * *valve_flow_rate.get(last_elem).unwrap() as i32;
            if flow_calc > max_flow.0 {
                max_flow = (flow_calc, path.clone());
            }
            let edges = valve_edges.get(last_elem).unwrap();
            for edge in edges {
                if !visited.contains(edge) {
                    let mut new_path = path.clone();
                    new_path.push(edge);
                    queue.push((distance - 1, new_path));
                }
            }
        }
        println!("Max flow for first is... {}", max_flow.0);
        for path_el in &max_flow.1 {
            println!("{}", path_el);
        }
        minutes_elapsed += max_flow.1.len() as i32 + 1;
    }
    Ok(())
}
