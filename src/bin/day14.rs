use std::{collections::VecDeque, io::BufRead};

type Point = (usize, usize);
type Range = (usize, usize);
enum Item {
    Air,
    Source,
    Rock,
    Sand,
}

#[derive(Default)]
struct Reservoir {
    grid: Vec<VecDeque<Item>>,
    source: Point,
    x_range: Range,
    y_range: Range,
    is_sand_free_fall: bool,
    num_sand_at_rest: usize,
}

impl Reservoir {
    pub fn new(source: Point) -> Reservoir {
        let x_range = (source.0, source.0 + 1);
        let y_range = (source.1, source.1 + 1);

        let mut vd = VecDeque::new();
        vd.push_back(Item::Source);
        let grid = vec![vd];
        return Reservoir {
            source: source,
            x_range: x_range,
            y_range: y_range,
            grid: grid,
            ..Default::default()
        };
    }

    fn height(&self) -> usize {
        return self.grid.len();
    }

    fn width(&self) -> usize {
        return self.grid[0].len();
    }

    pub fn to_str(&self) {
        for row in &self.grid {
            for col in row {
                if matches!(col, Item::Source) {
                    print!("+");
                } else if matches!(col, Item::Rock) {
                    print!("#");
                } else if matches!(col, Item::Sand) {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn resize(&mut self, pt: Point) {
        if self.x_range.0 > pt.0 {
            for row in &mut self.grid {
                for _ in pt.0..self.x_range.0 {
                    row.push_front(Item::Air);
                }
            }
            self.x_range.0 = pt.0;
        }

        if self.x_range.1 <= pt.0 {
            for row in &mut self.grid {
                for _ in self.x_range.1..pt.0 + 1 {
                    row.push_back(Item::Air);
                }
            }
            self.x_range.1 = pt.0 + 1;
        }

        // dont need to handle since y_range starts at 0
        if self.y_range.0 > pt.1 {}

        if self.y_range.1 <= pt.1 {
            for _ in self.y_range.1..pt.1 + 1 {
                let mut last_row = VecDeque::new();
                for _ in 0..self.width() {
                    last_row.push_back(Item::Air);
                }
                self.grid.push(last_row);
            }
            self.y_range.1 = pt.1 + 1;
        }
    }

    pub fn is_air_at(&self, pt: Point) -> bool {
        if pt.0 < self.x_range.0 || pt.0 >= self.x_range.1 || pt.1 >= self.y_range.1 {
            return false;
        }
        let col = pt.0 - self.x_range.0;
        let row = pt.1 - self.y_range.0;
        matches!(self.grid[row][col], Item::Air)
    }

    pub fn draw_item_at(&mut self, pt: Point, item: Item) {
        let col = pt.0 - self.x_range.0;
        let row = pt.1 - self.y_range.0;
        self.grid[row][col] = item;
    }

    pub fn draw_rock(&mut self, mut start: Point, end: Point) {
        self.resize(start);
        self.resize(end);
        while start.0 != end.0 {
            self.draw_item_at(start, Item::Rock);
            if end.0 > start.0 {
                start.0 += 1;
            } else if end.0 < start.0 {
                start.0 -= 1;
            }
        }

        while start.1 != end.1 {
            self.draw_item_at(start, Item::Rock);
            if end.1 > start.1 {
                start.1 += 1
            } else if end.1 < start.1 {
                start.1 -= 1;
            }
        }
        self.draw_item_at(end, Item::Rock);
    }

    pub fn drop_sand(&mut self) {
        let mut sand_pos = self.source;
        loop {
            if self.is_air_at((sand_pos.0, sand_pos.1 + 1)) {
                // try to move down
                sand_pos.1 += 1;
            } else if self.is_air_at((sand_pos.0 - 1, sand_pos.1 + 1)) {
                // try to move down left
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if self.is_air_at((sand_pos.0 + 1, sand_pos.1 + 1)) {
                // try to move down right
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                if sand_pos.1 >= self.height() - 1 {
                    self.is_sand_free_fall = true;
                } else {
                    self.draw_item_at(sand_pos, Item::Sand);
                    self.num_sand_at_rest += 1;
                }
                break;
            }
        }
    }
}

fn str_to_point(point_str: &str) -> Point {
    let coordinates: Vec<&str> = point_str.split(",").collect();
    let x = coordinates[0].parse::<usize>().ok().unwrap();
    let y = coordinates[1].parse::<usize>().ok().unwrap();
    (x, y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day14.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut reservoir = Reservoir::new((500, 0));
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let mut start = None;
        let mut points = line.split(" -> ");
        while let Some(point_str) = points.next() {
            if start.is_none() {
                start = Some(str_to_point(point_str));
            } else {
                let end = str_to_point(point_str);
                reservoir.draw_rock(start.unwrap(), end);
                start = Some(end);
            }
        }
    }

    while !reservoir.is_sand_free_fall {
        reservoir.drop_sand();
    }
    reservoir.to_str();
    println!("Sand at rest {}", reservoir.num_sand_at_rest);

    Ok(())
}
