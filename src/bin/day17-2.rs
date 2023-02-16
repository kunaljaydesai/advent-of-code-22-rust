use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

const DEBUG: bool = false;
const NUM_ROCKS: usize = 1000000000000;

#[derive(Clone)]
struct Rock {
    points: Vec<Vec<bool>>,
}

impl Rock {
    fn height(&self) -> usize {
        self.points.len()
    }
}

struct Board {
    width: usize,
    rocks: Vec<Rock>,
    state: HashMap<(usize, usize, [usize; 7]), Vec<(usize, usize)>>,
    rows: Vec<Vec<Option<usize>>>,
}

impl Board {
    fn new(width: usize) -> Board {
        let mut board = Board {
            width: width,
            rows: vec![],
            rocks: vec![],
            state: HashMap::new(),
        };
        board.add_blank_row();
        board.add_blank_row();
        board.add_blank_row();
        board
    }

    fn height(&self) -> usize {
        let mut height = 0;
        for row in &self.rows {
            let mut contains_rock = false;
            for rock in row {
                if !rock.is_none() {
                    contains_rock = true;
                    break;
                }
            }
            if contains_rock {
                height += 1;
            } else {
                return height;
            }
        }
        height
    }

    fn add_blank_row(&mut self) {
        let mut row = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            row.push(None);
        }
        self.rows.push(row);
    }

    fn add_rock(&mut self, rock: Rock) {
        let height = self.height();
        let rock_height = rock.height();

        if self.rows.len() < height + 3 + rock_height {
            for _ in self.rows.len()..height + 3 + rock_height {
                self.add_blank_row();
            }
        }
        let current_rock_idx = self.rocks.len();
        let mut row = 0;
        for rock_row in &rock.points {
            let mut col = 0;
            for is_rock in rock_row {
                if *is_rock {
                    self.rows[height + 3 + rock_height - row - 1][col + 2] = Some(current_rock_idx);
                }
                col += 1;
            }
            row += 1;
        }
        self.rocks.push(rock);
    }

    fn can_move_current_rock(&self, dx: isize, dy: isize) -> bool {
        let current_rock_idx = self.rocks.len() - 1;
        let mut row_idx: usize = 0;
        for row in &self.rows {
            let mut col_idx: usize = 0;
            for rock_idx_opt in row {
                if let Some(rock_idx) = rock_idx_opt {
                    if *rock_idx == current_rock_idx {
                        if let Some(new_row_idx) = row_idx.checked_add_signed(dy) {
                            if let Some(new_col_idx) = col_idx.checked_add_signed(dx) {
                                if new_col_idx >= 7 {
                                    return false;
                                }
                                if self.rows[new_row_idx][new_col_idx].is_some()
                                    && self.rows[new_row_idx][new_col_idx].unwrap() != *rock_idx
                                {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                col_idx += 1;
            }
            row_idx += 1;
        }
        return true;
    }

    fn move_current_rock(&mut self, dx: isize, dy: isize) {
        let current_rock_idx = self.rocks.len() - 1;
        let mut new_rows = self.rows.clone();
        let mut already_moved = HashSet::<(usize, usize)>::new();
        for row_idx in 0..self.rows.len() {
            for col_idx in 0..self.width {
                if let Some(val) = self.rows[row_idx][col_idx] {
                    if val == current_rock_idx {
                        if let Some(new_row_idx) = row_idx.checked_add_signed(dy) {
                            if let Some(new_col_idx) = col_idx.checked_add_signed(dx) {
                                if !already_moved.contains(&(row_idx, col_idx)) {
                                    new_rows[row_idx][col_idx] = None;
                                }
                                new_rows[new_row_idx][new_col_idx] = Some(current_rock_idx);
                                already_moved.insert((new_row_idx, new_col_idx));
                            }
                        }
                    }
                }
            }
        }
        self.rows = new_rows;
    }

    fn apply_wind(&mut self, c: char) {
        let mut dx = -1;
        if c == '>' {
            dx = 1;
        }
        if self.can_move_current_rock(dx, 0) {
            self.move_current_rock(dx, 0);
        }
    }

    fn get_border_trace(&self) -> [usize; 7] {
        let mut border_trace: [usize; 7] = Default::default();
        for idx in 0..self.width {
            let mut last_rock_idx = 0;
            let mut row_idx = 0;
            for row in &self.rows {
                if row[idx].is_some() {
                    last_rock_idx = row_idx;
                }
                row_idx += 1;
            }
            border_trace[idx] = self.rows.len() - last_rock_idx;
        }
        border_trace
    }
    fn save_state(&mut self, rock_idx: usize, wind_idx: usize) -> bool {
        let border_trace = self.get_border_trace();
        let height = self.height();
        if let Some(occurrences) = self.state.get_mut(&(rock_idx % 5, wind_idx, border_trace)) {
            if occurrences.len() >= 2 {
                if (NUM_ROCKS - occurrences[1].0) % (occurrences[1].0 - occurrences[0].0) == 0 {
                    let mut height = occurrences[1].1;
                    height += (NUM_ROCKS - occurrences[1].0)
                        / (occurrences[1].0 - occurrences[0].0)
                        * (occurrences[1].1 - occurrences[0].1);
                    println!("Height {}", height);
                    return true;
                }
            }
            occurrences.push((rock_idx, height));
        } else {
            self.state.insert(
                (rock_idx % 5, wind_idx, border_trace),
                vec![(rock_idx, height)],
            );
        }
        false
    }

    fn print(&self) {
        if !DEBUG {
            return;
        }
        for row in self.rows.iter().rev() {
            for col in row {
                if col.is_some() {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("")
        }
        println!("")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day17.txt");
    let file = std::fs::File::open(path)?;
    let mut lines = std::io::BufReader::new(file).lines();
    let line = lines.next().unwrap().ok().unwrap();
    let chars: Vec<char> = line.chars().collect();
    let mut rock_idx = 0;
    let mut board = Board::new(7);
    let rocks = [
        Rock {
            points: vec![vec![true, true, true, true]],
        },
        Rock {
            points: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
        },
        Rock {
            points: vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![true, true, true],
            ],
        },
        Rock {
            points: vec![vec![true], vec![true], vec![true], vec![true]],
        },
        Rock {
            points: vec![vec![true, true], vec![true, true]],
        },
    ];
    board.add_rock(rocks[rock_idx].clone());
    rock_idx += 1;
    board.print();
    let mut idx = 0;
    loop {
        let char = chars[idx % chars.len()];
        board.apply_wind(char);
        if DEBUG {
            println!("After applying wind {}", char);
        }
        board.print();
        if board.can_move_current_rock(0, -1) {
            board.move_current_rock(0, -1);
            if DEBUG {
                println!("After moving rock");
            }
            board.print();
        } else {
            if rock_idx == NUM_ROCKS {
                println!("Board height is {}", board.height());
                return Ok(());
            }
            if board.save_state(rock_idx, idx % chars.len()) {
                return Ok(());
            }
            board.add_rock(rocks[rock_idx % 5].clone());
            if DEBUG {
                println!("After adding new rock");
            }
            board.print();
            rock_idx += 1;
        }
        idx += 1;
        if DEBUG {
            if idx == 20 {
                break;
            }
        }
    }
    Ok(())
}
