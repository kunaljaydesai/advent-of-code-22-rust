use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn get_score_for(result: &str, opponent: &str) -> i32 {
    let is_opponent_rock = opponent.eq("A");
    let is_opponent_paper = opponent.eq("B");
    let is_opponent_scissors = opponent.eq("C");
    if result.eq("X") {
        // Lose
        if is_opponent_rock {
            /*
            Why do we need to write `as i32`?

            Action is an enum with 3 variants: Rock, Paper, Scissors. Each variant is of type Action so it needs to be cast
            to the correct type to satisfy the return tep of the `get_score_for` function.
            */
            return Action::Scissors as i32;
        } else if is_opponent_paper {
            return Action::Rock as i32;
        } else if is_opponent_scissors {
            return Action::Paper as i32;
        }
        return 0;
    } else if result.eq("Y") {
        // Tie
        let score = 3;
        if is_opponent_rock {
            return score + Action::Rock as i32;
        } else if is_opponent_paper {
            return score + Action::Paper as i32;
        } else if is_opponent_scissors {
            return score + Action::Scissors as i32;
        }
    } else if result.eq("Z") {
        let score = 6;
        if is_opponent_rock {
            return score + Action::Paper as i32;
        } else if is_opponent_paper {
            return score + Action::Scissors as i32;
        } else if is_opponent_scissors {
            return score + Action::Rock as i32;
        }
    }
    /*
    What is this random 0?
    If the last line of the function is just a value, it's implicitly a return statement.
    So, the below line is equivalent to return 0;
    */
    0
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("day2.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut score = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let mut split = line.split_whitespace();
        /*
        split.nth takes in a mutable str because it modifies the original string.
        If you call split.nth(0), it'll return the 0th element and remove it.
        It you call split.nth(1), it'll return the 1st element and remove it and all elements before it (0-1).
        */
        let opponent = split.nth(0).unwrap();
        let result = split.nth(0).unwrap();
        score += get_score_for(result, opponent);
    }

    /*
    What is the exclamation mark before println here?
    println is not a function - it's a macro. Macros use ! to distinguish themselves from normal method calls.

    What's the difference between a macro and a function?
    - Macros can take in a variable number of parameters
    - Macros are expanded before compile-time, functions are evaluated at compile time
    */
    println!("Score of strategy {:?}", score);
    Ok(())
}
