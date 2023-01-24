use std::{io::BufRead, ops::Index};

struct Forest {
    grid: Vec<Vec<u8>>,
}

impl Forest {
    pub fn add_tree_row(&mut self, row: &String) {
        let mut row_vec = vec![];
        for char in row.chars() {
            row_vec.push(char as u8 - '0' as u8);
        }
        self.grid.push(row_vec);
    }

    pub fn tree_score(&self, row: usize, col: usize) -> usize {
        let on_edge = row == 0
            || row == self.grid.len() - 1
            || col == 0
            || col == self.grid.index(0).len() - 1;
        if on_edge {
            return 0;
        }
        return self.tree_score_direction(row, col, -1, 0)
            * self.tree_score_direction(row, col, 0, -1)
            * self.tree_score_direction(row, col, 0, 1)
            * self.tree_score_direction(row, col, 1, 0);
    }

    pub fn tree_score_direction(
        &self,
        mut row: usize,
        mut col: usize,
        dx: isize,
        dy: isize,
    ) -> usize {
        let num_rows = self.grid.len();
        let num_cols = self.grid.index(0).len();
        let tree_height = *self.grid.index(row).index(col);
        let mut score = 0;
        if dx.is_negative() {
            if let Some(val) = row.checked_sub(dx.abs() as usize) {
                row = val;
            } else {
                return score;
            }
        } else {
            if let Some(val) = row.checked_add(dx.abs() as usize) {
                row = val;
            } else {
                return score;
            }
        }

        if dy.is_negative() {
            if let Some(val) = col.checked_sub(dy.abs() as usize) {
                col = val;
            } else {
                return score;
            }
        } else {
            if let Some(val) = col.checked_add(dy.abs() as usize) {
                col = val;
            } else {
                return score;
            }
        }

        while row < num_rows && col < num_cols {
            score += 1;
            if tree_height <= *self.grid.index(row).index(col) {
                return score;
            }
            if dx.is_negative() {
                if let Some(val) = row.checked_sub(dx.abs() as usize) {
                    row = val;
                } else {
                    return score;
                }
            } else {
                if let Some(val) = row.checked_add(dx.abs() as usize) {
                    row = val;
                } else {
                    return score;
                }
            }

            if dy.is_negative() {
                if let Some(val) = col.checked_sub(dy.abs() as usize) {
                    col = val;
                } else {
                    return score;
                }
            } else {
                if let Some(val) = col.checked_add(dy.abs() as usize) {
                    col = val;
                } else {
                    return score;
                }
            }
        }
        return score;
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day8.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut forest = Forest { grid: vec![] };
    for line in reader.lines() {
        forest.add_tree_row(&line.unwrap());
    }
    let mut max_score = 0;
    for row in 0..forest.grid.len() {
        for col in 0..forest.grid.index(0).len() {
            let val = forest.tree_score(row, col);
            if val > max_score {
                max_score = val;
            }
        }
    }
    println!("max tree score: {}", max_score);
    Ok(())
}
