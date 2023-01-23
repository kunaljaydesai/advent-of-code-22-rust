use std::{collections::HashSet, io::BufRead};

fn main() -> Result<(), std::io::Error> {
    let path = std::path::Path::new("day6.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let mut char_vec: Vec<char> = vec![];
        let mut count = 0;
        for c in line.unwrap().chars() {
            count += 1;
            char_vec.push(c);
            if char_vec.len() > 4 {
                char_vec.remove(0);
            }
            if count >= 4 {
                if HashSet::<char>::from_iter(char_vec.clone()).len() == 4 {
                    println!("Count {}", count);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}
