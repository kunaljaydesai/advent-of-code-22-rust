use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

const DIM: i32 = 20;

fn is_exposed_externally(
    point: (i32, i32, i32),
    point_set: &HashSet<(i32, i32, i32)>,
    cache: &mut HashMap<(i32, i32, i32), bool>,
) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(point);
    let mut seen_points = HashSet::new();
    while let Some(point) = queue.pop_front() {
        if seen_points.contains(&point) {
            continue;
        }
        seen_points.insert(point);
        if point.0 < 0
            || point.1 < 0
            || point.2 < 0
            || point.0 >= DIM
            || point.1 >= DIM
            || point.2 >= DIM
            || (cache.contains_key(&point) && *cache.get(&point).unwrap())
        {
            cache.insert(point, true);
            return true;
        }

        if point_set.contains(&point)
            || (cache.contains_key(&point) && !*cache.get(&point).unwrap())
        {
            continue;
        }

        for delta in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let adjacent_point = (delta.0 + point.0, delta.1 + point.1, delta.2 + point.2);
            queue.push_back(adjacent_point);
        }
    }
    cache.insert(point, false);
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = HashMap::<(i32, i32, i32), bool>::new();
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
            let adjacent_point = (delta.0 + point.0, delta.1 + point.1, delta.2 + point.2);
            if !points.contains(&adjacent_point)
                && is_exposed_externally(adjacent_point, &points, &mut cache)
            {
                num_exposed += 1;
            }
        }
    }
    println!("External surface area is {}", num_exposed);
    Ok(())
}
