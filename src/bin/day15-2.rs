use std::io::BufRead;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
        return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day15.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();

    let mut sensors = vec![];
    for line_result in lines {
        let line = line_result.ok().unwrap();

        // safe to add 2 because all characters here are ASCII characters
        let start_sensor_x = line.find("x=").unwrap() + 2;
        let end_sensor_x = line.find(',').unwrap();
        let sensor_x = (&line[start_sensor_x..end_sensor_x])
            .parse::<i32>()
            .ok()
            .unwrap();

        // safe to add 2 because all characters here are ASCII characters
        let start_sensor_y = line.find("y=").unwrap() + 2;
        let end_sensor_y = line.find(":").unwrap();
        let sensor_y = (&line[start_sensor_y..end_sensor_y])
            .parse::<i32>()
            .ok()
            .unwrap();

        let sensor_loc = Point {
            x: sensor_x,
            y: sensor_y,
        };

        let start_close_x = line.rfind("x=").unwrap() + 2;
        let end_close_x = line.rfind(',').unwrap();
        let close_x = (&line[start_close_x..end_close_x])
            .parse::<i32>()
            .ok()
            .unwrap();

        let start_close_y = line.rfind("y=").unwrap() + 2;
        let end_close_y = line.len();
        let close_y = (&line[start_close_y..end_close_y])
            .parse::<i32>()
            .ok()
            .unwrap();

        let close_sensor_loc = Point {
            x: close_x,
            y: close_y,
        };
        sensors.push((sensor_loc, close_sensor_loc));
    }
    let mut ranges: Vec<Vec<(i32, i32)>> = Vec::with_capacity(4_000_001);
    for _ in 0..4_000_001 {
        ranges.push(Vec::new());
    }

    for pair in sensors {
        let sensor = pair.0;
        let beacon = pair.1;
        let distance = Point::manhattan_distance(&sensor, &beacon);
        for dy in -distance..distance + 1 {
            let y = sensor.y + dy;
            if y < 0 || y > 4_000_000 {
                continue;
            }
            let dx = distance - dy.abs();
            let x_range = ((sensor.x - dx).max(0), (sensor.x + dx).min(4_000_000));
            ranges[y as usize].push(x_range);
            // ranges.get_mut(y).push(x_range);
        }
    }
    let mut y = 0;
    for intervals in &mut ranges {
        intervals.sort();
        let mut current_max = intervals[0].1;
        for interval_idx in 1..intervals.len() {
            let interval = intervals[interval_idx];
            if interval.0 > current_max + 1 {
                println!("found gap {}x{}", interval.0 - 1, y);
                return Ok(());
            }
            current_max = current_max.max(interval.1);
        }
        y += 1;
    }

    Ok(())
}
