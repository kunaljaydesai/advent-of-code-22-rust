use std::{cmp::min, collections::BinaryHeap, io::BufRead};

type Point = (i32, i32);

fn min_path(height_map: [[char; 101]; 41], end: Point, calculated_distance: &mut [[i32; 101]; 41]) {
    let mut queue = BinaryHeap::new();
    queue.push((0, end));
    const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while queue.len() > 0 {
        let item = queue.pop().unwrap();
        let point = item.1;
        let point_row = usize::try_from(point.0).ok().unwrap();
        let point_col = usize::try_from(point.1).ok().unwrap();
        let distance = item.0;
        calculated_distance[point_row][point_col] =
            std::cmp::min(distance, calculated_distance[point_row][point_col]);
        for delta in DELTAS {
            let to_point = (point.0 + delta.0, point.1 + delta.1);
            let in_bounds =
                to_point.0 >= 0 && to_point.0 < 41 && to_point.1 >= 0 && to_point.1 < 101;
            if !in_bounds {
                continue;
            }
            let to_row = usize::try_from(to_point.0).ok().unwrap();
            let to_col = usize::try_from(to_point.1).ok().unwrap();
            let valid_height =
                height_map[to_row][to_col] as u32 + 1 >= height_map[point_row][point_col] as u32;

            if !valid_height {
                continue;
            }

            let visited = calculated_distance[to_row][to_col] <= distance + 1;
            if visited {
                continue;
            }

            queue.push((distance + 1, to_point));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day12.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut calculated_distance = [[std::i32::MAX; 101]; 41];
    let mut height_map = [['0'; 101]; 41];
    let mut row = 0;
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);
    for line_result in reader.lines() {
        let line = line_result?;
        let mut col = 0;
        for char in line.chars() {
            if char == 'S' {
                start = (row, col);
                height_map[usize::try_from(row).ok().unwrap()]
                    [usize::try_from(col).ok().unwrap()] = 'a';
            } else if char == 'E' {
                end = (row, col);
                height_map[usize::try_from(row).ok().unwrap()]
                    [usize::try_from(col).ok().unwrap()] = 'z';
            } else {
                height_map[usize::try_from(row).ok().unwrap()]
                    [usize::try_from(col).ok().unwrap()] = char;
            }
            col += 1;
        }
        row += 1;
    }
    min_path(height_map, end, &mut calculated_distance);
    let mut min_distance = std::i32::MAX;
    for row in 0..41 {
        for col in 0..101 {
            if height_map[row][col] == 'a' {
                let distance = calculated_distance[row][col];
                min_distance = std::cmp::min(distance, min_distance);
            }
        }
    }
    println!("Min distance {}", min_distance);
    Ok(())
}
