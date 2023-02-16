use std::{collections::HashSet, io::BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day18.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut points = HashSet::new();
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let dims: Vec<&str> = line.split(",").collect();
        let x = dims[0].parse::<i32>()?;
        let y = dims[1].parse::<i32>()?;
        let z = dims[2].parse::<i32>()?;
        points.insert((x, y, z));
    }
    let mut num_exposed = 0;
    for point in &points {
        for delta in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if !points.contains(&(delta.0 + point.0, delta.1 + point.1, delta.2 + point.2)) {
                num_exposed += 1;
            }
        }
    }
    println!("Surface area is {}", num_exposed);
    Ok(())
}
