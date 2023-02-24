use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{io::BufRead, ops::Add};

type Position = (usize, usize);

#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: Position,
    grid: Vec<Vec<Vec<char>>>,
    minutes_elapsed: usize,
    goal: Position,
}

impl State {
    fn distance_from_goal(&self) -> usize {
        return self
            .pos
            .0
            .abs_diff(self.goal.0)
            .add(self.pos.1.abs_diff(self.goal.1));
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn get_next_blizzard_col(&self, c_idx: usize, dir: isize) -> usize {
        if c_idx == 1 && dir == -1 {
            return self.width() - 2;
        }
        if c_idx == self.width() - 2 && dir == 1 {
            return 1;
        }
        return c_idx.checked_add_signed(dir).unwrap();
    }

    fn get_next_blizzard_row(&self, r_idx: usize, dir: isize) -> usize {
        if r_idx == 1 && dir == -1 {
            return self.height() - 2;
        }
        if r_idx == self.height() - 2 && dir == 1 {
            return 1;
        }
        return r_idx.checked_add_signed(dir).unwrap();
    }

    fn simulate_blizzard(&mut self) {
        let mut new_grid = vec![];
        for _ in 0..self.height() {
            new_grid.push(vec![]);
            let last_idx = new_grid.len() - 1;
            for _ in 0..self.width() {
                new_grid[last_idx].push(vec![]);
            }
        }
        for r_idx in 0..self.grid.len() {
            let row = self.grid[r_idx].clone();
            for c_idx in 0..row.len() {
                let spot = row[c_idx].clone();
                for idx in 0..spot.len() {
                    let elem = spot[idx];
                    match elem {
                        '>' => new_grid[r_idx][self.get_next_blizzard_col(c_idx, 1)].push('>'),
                        '^' => new_grid[self.get_next_blizzard_row(r_idx, -1)][c_idx].push('^'),
                        '<' => new_grid[r_idx][self.get_next_blizzard_col(c_idx, -1)].push('<'),
                        'v' => new_grid[self.get_next_blizzard_row(r_idx, 1)][c_idx].push('v'),
                        '#' => new_grid[r_idx][c_idx].push('#'),
                        '.' => new_grid[r_idx][c_idx].push('.'),
                        _ => {
                            panic!("Invalid char...");
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn is_valid(&self, delta: (isize, isize)) -> bool {
        if delta.0 < 0 && self.pos.0 == 0 {
            return false;
        }

        if delta.0 > 0 && self.pos.0 == self.height() - 1 {
            return false;
        }

        if delta.1 < 0 && self.pos.1 == 0 {
            return false;
        }

        if delta.1 > 0 && self.pos.1 == self.width() - 1 {
            return false;
        }

        let new_row = self.pos.0.checked_add_signed(delta.0).unwrap();
        let new_col = self.pos.1.checked_add_signed(delta.1).unwrap();
        let elems = &self.grid[new_row][new_col];
        let is_valid = !elems.contains(&'v')
            && !elems.contains(&'^')
            && !elems.contains(&'>')
            && !elems.contains(&'<')
            && !elems.contains(&'#');
        is_valid
    }

    fn get_valid_positions(&self) -> Vec<Position> {
        let mut pos = vec![];

        if self.is_valid((0, 0)) {
            pos.push(self.pos);
        }

        if self.is_valid((1, 0)) {
            pos.push((self.pos.0 + 1, self.pos.1));
        }

        if self.is_valid((-1, 0)) {
            pos.push((self.pos.0 - 1, self.pos.1));
        }

        if self.is_valid((0, -1)) {
            pos.push((self.pos.0, self.pos.1 - 1));
        }

        if self.is_valid((0, 1)) {
            pos.push((self.pos.0, self.pos.1 + 1));
        }

        pos
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.distance_from_goal())
            .cmp(&self.distance_from_goal())
            .then_with(|| other.minutes_elapsed.cmp(&self.minutes_elapsed))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse
    let path = std::path::Path::new("day24.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut grid = vec![];
    for line_result in lines {
        let line = line_result.ok().unwrap();
        grid.push(vec![]);
        let last_idx = grid.len() - 1;
        for char in line.chars() {
            grid[last_idx].push(vec![char]);
        }
    }
    let mut grid_cache = HashMap::<usize, Vec<Vec<Vec<char>>>>::new();
    let start = (0, 1);
    let mut pq = BinaryHeap::new();
    pq.push(State {
        pos: start,
        grid: grid.clone(),
        minutes_elapsed: 0,
        goal: (grid.len() - 1, grid[0].len() - 2),
    });

    let end = (grid.len() - 1, grid[0].len() - 2);
    let mut min_time_to_goal = usize::MAX;
    let mut visited = HashSet::new();
    let mut next_start_state = None;
    while let Some(mut s) = pq.pop() {
        if s.pos == end {
            if s.minutes_elapsed < min_time_to_goal {
                next_start_state = Some(s.clone());
                min_time_to_goal = s.minutes_elapsed;
            }
            continue;
        }
        if s.minutes_elapsed >= min_time_to_goal - 1 {
            continue;
        }

        if s.distance_from_goal() >= min_time_to_goal - s.minutes_elapsed {
            continue;
        }

        if let Some(updated_grid) = grid_cache.get(&s.minutes_elapsed) {
            s.grid = updated_grid.clone();
        } else {
            s.simulate_blizzard();
            grid_cache.insert(s.minutes_elapsed, s.grid.clone());
        }
        s.minutes_elapsed += 1;
        for pos in s.get_valid_positions() {
            let mut new_state = s.clone();
            new_state.pos = pos;
            if !visited.contains(&(s.minutes_elapsed, pos)) {
                visited.insert((s.minutes_elapsed, pos));
                pq.push(new_state);
            }
        }
    }
    let mut curr_time = min_time_to_goal;
    println!("got curr time {}", curr_time);
    let mut next_state = next_start_state.unwrap();
    let mut next_start_state = None;
    next_state.goal = (0, 1);
    next_state.minutes_elapsed = 0;
    pq.push(next_state);
    min_time_to_goal = usize::MAX;
    visited.clear();
    grid_cache.clear();
    while let Some(mut s) = pq.pop() {
        if s.pos == start {
            if s.minutes_elapsed < min_time_to_goal {
                next_start_state = Some(s.clone());
                min_time_to_goal = s.minutes_elapsed;
            }
            continue;
        }
        if s.minutes_elapsed >= min_time_to_goal - 1 {
            continue;
        }

        if s.distance_from_goal() >= min_time_to_goal - s.minutes_elapsed {
            continue;
        }

        if let Some(updated_grid) = grid_cache.get(&s.minutes_elapsed) {
            s.grid = updated_grid.clone();
        } else {
            s.simulate_blizzard();
            grid_cache.insert(s.minutes_elapsed, s.grid.clone());
        }
        s.minutes_elapsed += 1;
        for pos in s.get_valid_positions() {
            let mut new_state = s.clone();
            new_state.pos = pos;
            if !visited.contains(&(s.minutes_elapsed, pos)) {
                visited.insert((s.minutes_elapsed, pos));
                pq.push(new_state);
            }
        }
    }
    curr_time += min_time_to_goal;

    println!("got curr time {}", curr_time);
    next_state = next_start_state.unwrap();
    next_state.goal = (grid.len() - 1, grid[0].len() - 2);
    next_state.minutes_elapsed = 0;
    pq.push(next_state);
    min_time_to_goal = usize::MAX;
    visited.clear();
    grid_cache.clear();
    while let Some(mut s) = pq.pop() {
        if s.pos == end {
            if s.minutes_elapsed < min_time_to_goal {
                next_start_state = Some(s.clone());
                min_time_to_goal = s.minutes_elapsed;
            }
            continue;
        }
        if s.minutes_elapsed >= min_time_to_goal - 1 {
            continue;
        }

        if s.distance_from_goal() >= min_time_to_goal - s.minutes_elapsed {
            continue;
        }

        if let Some(updated_grid) = grid_cache.get(&s.minutes_elapsed) {
            s.grid = updated_grid.clone();
        } else {
            s.simulate_blizzard();
            grid_cache.insert(s.minutes_elapsed, s.grid.clone());
        }
        s.minutes_elapsed += 1;
        for pos in s.get_valid_positions() {
            let mut new_state = s.clone();
            new_state.pos = pos;
            if !visited.contains(&(s.minutes_elapsed, pos)) {
                visited.insert((s.minutes_elapsed, pos));
                pq.push(new_state);
            }
        }
    }
    curr_time += min_time_to_goal;
    println!("min time to goal {}", curr_time);
    Ok(())
}
