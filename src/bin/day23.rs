use core::num;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

type Point = (i32, i32);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day23.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut elf_locations = HashSet::<Point>::new();
    let mut r_idx = 0;
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let mut c_idx = 0;
        for char in line.chars() {
            if char == '#' {
                elf_locations.insert((r_idx, c_idx));
            }
            c_idx += 1;
        }
        r_idx += 1;
    }
    let mut directions = VecDeque::new();
    directions.push_back('N');
    directions.push_back('S');
    directions.push_back('W');
    directions.push_back('E');
    for _ in 0..10 {
        // first half
        let mut proposals = HashMap::<Point, Point>::new();
        let mut destinations = HashMap::<Point, Point>::new();
        for (r_idx_ref, c_idx_ref) in &elf_locations {
            let r_idx = *r_idx_ref;
            let c_idx = *c_idx_ref;
            // check if there is an elf in elf in an adjacent position
            let mut is_elf_adjacent = false;
            for dr in -1..2 {
                for dc in -1..2 {
                    if !(dr == 0 && dc == 0) && elf_locations.contains(&(r_idx + dr, c_idx + dc)) {
                        is_elf_adjacent = true;
                    }
                }
            }
            if !is_elf_adjacent {
                continue;
            }

            let source = (r_idx, c_idx);
            for direction in &directions {
                let dir_char = *direction;
                match dir_char {
                    'N' => {
                        let mut is_elf_north = false;
                        let adj_r_idx = r_idx - 1;
                        for dc in -1..2 {
                            if elf_locations.contains(&(adj_r_idx, c_idx + dc)) {
                                is_elf_north = true;
                            }
                        }
                        if !is_elf_north {
                            let destination = (source.0 - 1, source.1);
                            if destinations.contains_key(&destination) {
                                proposals.remove(destinations.get(&destination).unwrap());
                            } else {
                                proposals.insert(source, destination);
                                destinations.insert(destination, source);
                            }
                            break;
                        }
                    }
                    'S' => {
                        let mut is_elf_south = false;
                        let adj_r_idx = r_idx + 1;
                        for dc in -1..2 {
                            if elf_locations.contains(&(adj_r_idx, c_idx + dc)) {
                                is_elf_south = true;
                            }
                        }
                        if !is_elf_south {
                            let destination = (source.0 + 1, source.1);
                            if destinations.contains_key(&destination) {
                                proposals.remove(destinations.get(&destination).unwrap());
                            } else {
                                proposals.insert(source, destination);
                                destinations.insert(destination, source);
                            }
                            break;
                        }
                    }
                    'E' => {
                        let mut is_elf_east = false;
                        let adj_c_idx = c_idx + 1;
                        for dr in -1..2 {
                            if elf_locations.contains(&(r_idx + dr, adj_c_idx)) {
                                is_elf_east = true;
                            }
                        }
                        if !is_elf_east {
                            let destination = (source.0, source.1 + 1);
                            if destinations.contains_key(&destination) {
                                proposals.remove(destinations.get(&destination).unwrap());
                            } else {
                                proposals.insert(source, destination);
                                destinations.insert(destination, source);
                            }
                            break;
                        }
                    }
                    'W' => {
                        let mut is_elf_west = false;
                        let adj_c_idx = c_idx - 1;
                        for dr in -1..2 {
                            if elf_locations.contains(&(r_idx + dr, adj_c_idx)) {
                                is_elf_west = true;
                            }
                        }
                        if !is_elf_west {
                            let destination = (source.0, source.1 - 1);
                            if destinations.contains_key(&destination) {
                                proposals.remove(destinations.get(&destination).unwrap());
                            } else {
                                proposals.insert(source, destination);
                                destinations.insert(destination, source);
                            }
                            break;
                        }
                    }
                    _ => {
                        panic!("invalid direction");
                    }
                }
            }
        }

        // second half - actually move the elves
        for proposal in proposals {
            if !elf_locations.remove(&proposal.0) {
                panic!("error...");
            }
            elf_locations.insert(proposal.1);
        }

        // re-order directions
        let front = directions.pop_front().unwrap();
        directions.push_back(front);
    }

    // calculate score
    let mut col_bounds = (std::i32::MAX, std::i32::MIN);
    let mut row_bounds = (std::i32::MAX, std::i32::MIN);
    for location in &elf_locations {
        row_bounds = (row_bounds.0.min(location.0), row_bounds.1.max(location.0));
        col_bounds = (col_bounds.0.min(location.1), col_bounds.1.max(location.1));
    }
    println!(
        "score: {}",
        ((col_bounds.1 - col_bounds.0 + 1) * (row_bounds.1 - row_bounds.0 + 1))
            - i32::try_from(elf_locations.len()).ok().unwrap()
    );
    Ok(())
}
