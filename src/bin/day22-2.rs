use std::io::BufRead;

enum Direction {
    Magnitude(u32),
    Rotation(char),
}

enum PosType {
    Wall,
    Open,
}

type Position = (i32, i32, i32, char);
fn get_pos_type(pos: Position, sides: &[Vec<Vec<char>>; 6]) -> PosType {
    let r_idx = pos.0;
    let c_idx = pos.1;
    let val = sides[usize::try_from(pos.2).ok().unwrap()][usize::try_from(r_idx).ok().unwrap()]
        [usize::try_from(c_idx).ok().unwrap()];
    if val == '#' {
        return PosType::Wall;
    } else if val == '.' {
        return PosType::Open;
    }
    panic!("Invalid pos");
}

fn calc_next_pos(pos: Position) -> Position {
    let mut delta = (0, 0);
    match pos.3 {
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
    let side = pos.2;
    if pos.0 + delta.0 < 0 {
        // up
        assert!('U' == pos.3);
        match side {
            0 => {
                return (pos.1, 0, 5, 'R');
            }
            1 => {
                return (49, pos.1, 5, 'U');
            }
            2 => {
                return (49, pos.1, 0, 'U');
            }
            3 => {
                return (pos.1, 0, 2, 'R');
            }
            4 => {
                return (49, pos.1, 2, 'U');
            }
            5 => {
                return (49, pos.1, 3, 'U');
            }
            _ => {
                panic!("Invalid...")
            }
        }
    } else if pos.0 + delta.0 >= 50 {
        // down
        assert!('D' == pos.3);
        match side {
            0 => {
                return (0, pos.1, 2, 'D');
            }
            1 => {
                return (pos.1, 49, 2, 'L');
            }
            2 => {
                return (0, pos.1, 4, 'D');
            }
            3 => {
                return (0, pos.1, 5, 'D');
            }
            4 => {
                return (pos.1, 49, 5, 'L');
            }
            5 => {
                return (0, pos.1, 1, 'D');
            }
            _ => {
                panic!("Invalid...")
            }
        }
    } else if pos.1 + delta.1 < 0 {
        // left
        assert!('L' == pos.3);
        match side {
            0 => {
                return (pos.0.abs_diff(49).try_into().unwrap(), 0, 3, 'R');
            }
            1 => {
                return (pos.0, 49, 0, 'L');
            }
            2 => {
                return (0, pos.0, 3, 'D');
            }
            3 => {
                return (pos.0.abs_diff(49).try_into().unwrap(), 0, 0, 'R');
            }
            4 => {
                return (pos.0, 49, 3, 'L');
            }
            5 => {
                return (0, pos.0, 0, 'D');
            }
            _ => {
                panic!("Invalid...")
            }
        }
    } else if pos.1 + delta.1 >= 50 {
        assert!('R' == pos.3);
        // right
        match side {
            0 => {
                return (pos.0, 0, 1, 'R');
            }
            1 => {
                return (pos.0.abs_diff(49).try_into().unwrap(), 49, 4, 'L');
            }
            2 => {
                return (49, pos.0, 1, 'U');
            }
            3 => {
                return (pos.0, 0, 4, 'R');
            }
            4 => {
                return (pos.0.abs_diff(49).try_into().unwrap(), 49, 1, 'L');
            }
            5 => {
                return (49, pos.0, 4, 'U');
            }
            _ => {
                panic!("Invalid...")
            }
        }
    } else {
        return ((pos.0 + delta.0), (pos.1 + delta.1), pos.2, pos.3);
    }
}

fn get_next_pos(pos: Position, sides: &[Vec<Vec<char>>; 6]) -> Position {
    let next_pos = calc_next_pos(pos);
    match get_pos_type(next_pos, sides) {
        PosType::Wall => {
            return pos;
        }
        PosType::Open => {
            return next_pos;
        }
    }
}

fn move_pos(mut pos: Position, magnitude: u32, sides: &[Vec<Vec<char>>; 6]) -> Position {
    for _ in 0..magnitude {
        pos = get_next_pos(pos, sides);
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

    let mut sides = [vec![], vec![], vec![], vec![], vec![], vec![]];
    for r_idx in 0..board.len() {
        let row = &board[r_idx];
        for c_idx in 0..row.len() {
            if r_idx < 50 && c_idx >= 50 && c_idx < 100 {
                if sides[0].len() <= r_idx {
                    sides[0].push(vec![]);
                }
                sides[0][r_idx].push(row[c_idx]);
            } else if r_idx < 50 && c_idx >= 100 && c_idx < 150 {
                if sides[1].len() <= r_idx {
                    sides[1].push(vec![]);
                }
                sides[1][r_idx].push(row[c_idx]);
            } else if r_idx >= 50 && r_idx < 100 && c_idx >= 50 && c_idx < 100 {
                if sides[2].len() <= r_idx - 50 {
                    sides[2].push(vec![]);
                }
                sides[2][r_idx - 50].push(row[c_idx]);
            } else if r_idx >= 100 && r_idx < 150 && c_idx < 50 {
                if sides[3].len() <= r_idx - 100 {
                    sides[3].push(vec![]);
                }
                sides[3][r_idx - 100].push(row[c_idx]);
            } else if r_idx >= 100 && r_idx < 150 && c_idx >= 50 && c_idx < 100 {
                if sides[4].len() <= r_idx - 100 {
                    sides[4].push(vec![]);
                }
                sides[4][r_idx - 100].push(row[c_idx]);
            } else if r_idx >= 150 && r_idx < 200 && c_idx < 50 {
                if sides[5].len() <= r_idx - 150 {
                    sides[5].push(vec![]);
                }
                sides[5][r_idx - 150].push(row[c_idx]);
            }
        }
    }

    for side in &sides {
        for row in side {
            assert!(row.len() == 50);
            for c in row {
                assert!(*c == '.' || *c == '#');
            }
        }
        assert!(side.len() == 50);
    }

    let mut current_position: Position = (0, 0, 0, 'R');
    let mut idx = 0;
    for direction in directions {
        match direction {
            Direction::Magnitude(m) => {
                // println!(
                //     "current pos {}, {}, {}, {} magn: {}",
                //     current_position.0,
                //     current_position.1,
                //     current_position.2,
                //     current_position.3,
                //     m
                // );
                current_position = move_pos(current_position, m, &sides);
                // println!(
                //     "after pos {}, {}, {}, {}",
                //     current_position.0, current_position.1, current_position.2, current_position.3
                // );
            }
            Direction::Rotation(r) => {
                // println!("rotate {}", r);
                if r == 'R' {
                    if current_position.3 == 'U' {
                        current_position.3 = 'R';
                    } else if current_position.3 == 'D' {
                        current_position.3 = 'L';
                    } else if current_position.3 == 'R' {
                        current_position.3 = 'D';
                    } else if current_position.3 == 'L' {
                        current_position.3 = 'U';
                    }
                } else if r == 'L' {
                    if current_position.3 == 'U' {
                        current_position.3 = 'L';
                    } else if current_position.3 == 'D' {
                        current_position.3 = 'R';
                    } else if current_position.3 == 'R' {
                        current_position.3 = 'U';
                    } else if current_position.3 == 'L' {
                        current_position.3 = 'D';
                    }
                }
            }
        }
        // if idx > 7 {
        //     break;
        // }
        idx += 1;
    }
    let mut facing_score = 0;
    match current_position.3 {
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
    let mut row_add = 0;
    let mut col_add = 0;
    match current_position.2 {
        0 => {
            row_add = 0;
            col_add = 50;
        }
        1 => {
            row_add = 0;
            col_add = 100;
        }
        2 => {
            row_add = 50;
            col_add = 50;
        }
        3 => {
            row_add = 100;
            col_add = 0;
        }
        4 => {
            row_add = 100;
            col_add = 50;
        }
        5 => {
            row_add = 150;
            col_add = 0;
        }
        _ => {
            panic!("Invalid side");
        }
    }

    println!(
        "score: {}",
        1000 * (current_position.0 + 1 + row_add) + 4 * (current_position.1 + 1 + col_add) + facing_score
    );
    Ok(())
}
