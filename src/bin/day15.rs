use core::num;
use std::{borrow::Borrow, io::BufRead, ops::Index};

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
    let mut x_range = (std::i32::MAX, std::i32::MIN);
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
        x_range.0 = (sensor_loc.x - Point::manhattan_distance(&sensor_loc, &close_sensor_loc))
            .min(x_range.0);
        x_range.1 = (sensor_loc.x + Point::manhattan_distance(&sensor_loc, &close_sensor_loc))
            .max(x_range.1);
        sensors.push((sensor_loc, close_sensor_loc));
    }
    let mut num_cant_contain = 0;
    for x in x_range.0..x_range.1 + 1 {
        let mut can_contain = true;
        for pair in &sensors {
            let sensor = &pair.0;
            let beacon = &pair.1;
            let distance = Point::manhattan_distance(sensor, beacon);
            let pt = &Point { x: x, y: 2000000 };
            if distance >= Point::manhattan_distance(sensor, pt)
                && !(beacon.x == pt.x && beacon.y == pt.y)
            {
                can_contain = false;
                break;
            }
        }
        if !can_contain {
            num_cant_contain += 1;
        }
    }
    println!("num cant contain {}", num_cant_contain);
    Ok(())
}
