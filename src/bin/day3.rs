use std::io::BufRead;
fn main() -> Result<(), std::io::Error> {
    let path = std::path::Path::new("day3.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut score = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let line_length = line.len();
        /*
        .chars() returns an object with the iterator trait.
        .take() and .collect() are methods by the iterator trait
        */
        let first_compartment: String = line.chars().take(line_length / 2).collect();
        let second_compartment: String = line
            .chars()
            .skip(line_length / 2)
            .take(line_length / 2)
            .collect();
        /*
        second_compartment_set needs to be declared `mut` since .extend accepts only a mutable reference.
        .extend under the hood mutates the hashset.
        */
        let mut second_compartment_set = std::collections::HashSet::<char>::new();
        second_compartment_set.extend(second_compartment.chars());
        for a in first_compartment.chars() {
            if second_compartment_set.contains(&a) {
                if a.is_ascii_lowercase() {
                    /*
                    as u32 converts the char into a number.
                    The number it is converted into is based on the ASCII table.
                    We subtract a value here to get the priority.
                    */
                    score += a as u32 - 96;
                } else if a.is_ascii_uppercase() {
                    score += a as u32 - 38;
                }
                break;
            }
        }
    }
    println!("Priority score is {:?}", score);
    Ok(())
}
