use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("day2.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut score = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let mut split = line.split_whitespace();
        let opponent = split.nth(0).unwrap();
        let response = split.nth(0).unwrap();

        if response.eq("X") {
            score += 1;
            if opponent.eq("C") {
                score += 6;
            } else if opponent.eq("A") {
                score += 3;
            }
        } else if response.eq("Y") {
            score += 2;
            if opponent.eq("A") {
                score += 6;
            } else if opponent.eq("B") {
                score += 3;
            }
        } else if response.eq("Z") {
            score += 3;
            if opponent.eq("B") {
                score += 6;
            } else if opponent.eq("C") {
                score += 3;
            }
        }
    }
    println!("Score of strategy {:?}", score);
    Ok(())
}
