use std::{io::BufRead, ops::Rem};

enum Direction {
    Magnitude(u32),
    Rotation(char),
}

enum PosType {
    Edge,
    Wall,
    Open,
}

type Position = (i32, i32, char);
fn get_pos_type(pos: Position, board: &Vec<Vec<char>>) -> PosType {
    let r_idx = pos.0;
    let c_idx = pos.1;
    if r_idx < 0
        || c_idx < 0
        || r_idx >= board.len().try_into().ok().unwrap()
        || c_idx >= board[0].len().try_into().ok().unwrap()
    {
        return PosType::Edge;
    }
    let val = board[usize::try_from(r_idx).ok().unwrap()][usize::try_from(c_idx).ok().unwrap()];
    if val == '#' {
        return PosType::Wall;
    } else if val == ' ' {
        return PosType::Edge;
    }
    return PosType::Open;
}

fn calc_next_pos(pos: Position, board: &Vec<Vec<char>>) -> Position {
    let mut delta = (0, 0);
    match pos.2 {
        'U' => {
            delta = (-1, 0);
        }
        'D' => {
            delta = (1, 0);
        }
        'R' => {
            delta = (0, 1);
        }
        'L' => {
            delta = (0, -1);
        }
        _ => {
            panic!("Invalid direction...")
        }
    }
    return (
        (pos.0 + delta.0).rem_euclid(board.len().try_into().ok().unwrap()),
        (pos.1 + delta.1).rem_euclid(board[0].len().try_into().ok().unwrap()),
        pos.2,
    );
}
fn get_next_pos(pos: Position, board: &Vec<Vec<char>>) -> Position {
    let mut next_pos = calc_next_pos(pos, board);
    match get_pos_type(next_pos, board) {
        PosType::Wall => {
            return pos;
        }
        PosType::Open => {
            return next_pos;
        }
        PosType::Edge => {
            next_pos = calc_next_pos(next_pos, board);
            while matches!(get_pos_type(next_pos, board), PosType::Edge) {
                next_pos = calc_next_pos(next_pos, board);
            }
            if matches!(get_pos_type(next_pos, board), PosType::Wall) {
                return pos;
            } else {
                return next_pos;
            }
        }
    }
}

fn move_pos(mut pos: Position, magnitude: u32, board: &Vec<Vec<char>>) -> Position {
    for _ in 0..magnitude {
        pos = get_next_pos(pos, board);
    }
    return pos;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day22.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut board = vec![];
    let mut is_next_directions = false;
    let mut directions = vec![];
    for line_result in lines {
        let line = line_result.ok().unwrap();
        if is_next_directions {
            let mut chars = line.chars();
            let mut curr_num = 0;
            while let Some(char) = chars.next() {
                if let Some(digit) = char::to_digit(char, 10) {
                    curr_num = 10 * curr_num + digit;
                } else {
                    directions.push(Direction::Magnitude(curr_num));
                    directions.push(Direction::Rotation(char));
                    curr_num = 0;
                }
            }
            directions.push(Direction::Magnitude(curr_num));
            continue;
        }
        if line.len() == 0 {
            is_next_directions = true;
            continue;
        }
        let last_idx = board.len();
        board.push(vec![]);
        for char in line.chars() {
            board[last_idx].push(char);
        }
    }
    let mut max_len = std::usize::MIN;
    for row in &board {
        max_len = max_len.max(row.len());
    }

    for row in &mut board {
        for _ in row.len()..max_len {
            row.push(' ');
        }
    }

    let mut current_position: (i32, i32, char) = (0, 0, 'R');
    'outer: for r_idx in 0..board.len() {
        let row = &board[r_idx];
        for c_idx in 0..row.len() {
            let item = row[c_idx];
            let is_open_spot = item == '.';
            if is_open_spot {
                current_position = (
                    r_idx.try_into().ok().unwrap(),
                    c_idx.try_into().ok().unwrap(),
                    'R',
                );
                break 'outer;
            }
        }
    }
    for direction in directions {
        match direction {
            Direction::Magnitude(m) => {
                current_position = move_pos(current_position, m, &board);
            }
            Direction::Rotation(r) => {
                if r == 'R' {
                    if current_position.2 == 'U' {
                        current_position.2 = 'R';
                    } else if current_position.2 == 'D' {
                        current_position.2 = 'L';
                    } else if current_position.2 == 'R' {
                        current_position.2 = 'D';
                    } else if current_position.2 == 'L' {
                        current_position.2 = 'U';
                    }
                } else if r == 'L' {
                    if current_position.2 == 'U' {
                        current_position.2 = 'L';
                    } else if current_position.2 == 'D' {
                        current_position.2 = 'R';
                    } else if current_position.2 == 'R' {
                        current_position.2 = 'U';
                    } else if current_position.2 == 'L' {
                        current_position.2 = 'D';
                    }
                }
            }
        }
    }
    let mut facing_score = 0;
    match current_position.2 {
        'D' => {
            facing_score = 1;
        }
        'R' => {
            facing_score = 0;
        }
        'L' => {
            facing_score = 2;
        }
        'U' => {
            facing_score = 3;
        }
        _ => {
            panic!("Error...");
        }
    }
    println!(
        "score: {}",
        1000 * (current_position.0 + 1) + 4 * (current_position.1 + 1) + facing_score
    );
    Ok(())
}
