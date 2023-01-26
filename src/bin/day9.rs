use std::{io::BufRead, ops::Index};

type Point = (i32, i32);

fn distance(pt1: Point, pt2: Point) -> i32 {
    return (pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs();
}

fn update_pos(pt: Point, dx: i32, dy: i32) -> Point {
    return (pt.0 + dx, pt.1 + dy);
}

fn is_touching(pt1: Point, pt2: Point) -> bool {
    for dx in -1..2 {
        for dy in -1..2 {
            if pt1.0 + dx == pt2.0 && pt1.1 + dy == pt2.1 {
                return true;
            }
        }
    }
    return false;
}

fn move_diagnol_towards(from: Point, to: Point) -> Point {
    let deltas = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut min_distance = std::i32::MAX;
    let mut min_delta = (0, 0);
    for delta in deltas {
        let new_pos = (from.0 + delta.0, from.1 + delta.1);
        let new_distance = distance(new_pos, to);
        if new_distance < min_distance {
            min_distance = new_distance;
            min_delta = (delta.0, delta.1);
        }
    }
    return update_pos(from, min_delta.0, min_delta.1);
}

fn get_num(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day9.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut visited = std::collections::HashSet::<Point>::new();
    let mut head_loc = (0, 0);
    let mut tail_loc = (0, 0);
    visited.insert(tail_loc);
    for line_result in reader.lines() {
        let line = line_result?;
        let direction = line.chars().nth(0).unwrap();
        let mut dx = 0;
        let mut dy = 0;
        if direction == 'U' {
            dy = 1;
        } else if direction == 'D' {
            dy = -1;
        } else if direction == 'R' {
            dx = 1;
        } else if direction == 'L' {
            dx = -1;
        }
        let steps = get_num(&line[2..]);
        for _ in 0..steps {
            // println!(
            //     "head ({}, {}), tail ({}, {})",
            //     head_loc.0, head_loc.1, tail_loc.0, tail_loc.1
            // );
            head_loc = update_pos(head_loc, dx, dy);
            let touching = is_touching(head_loc, tail_loc);
            let is_same_row_or_column = head_loc.0 == tail_loc.0 || head_loc.1 == tail_loc.1;
            if is_same_row_or_column && distance(head_loc, tail_loc) == 2 {
                tail_loc = update_pos(tail_loc, dx, dy);
            } else if !is_same_row_or_column && !touching {
                tail_loc = move_diagnol_towards(tail_loc, head_loc);
            }
            if !is_touching(head_loc, tail_loc) {
                println!("ERROR");
            }
            visited.insert(tail_loc);
        }
    }
    println!("Num positions {}", visited.len());
    Ok(())
}
