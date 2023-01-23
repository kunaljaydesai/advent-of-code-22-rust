use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let path = std::path::Path::new("day3.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut score = 0;
    let mut line_num = 0;

    /*
    Why do these need to be &mut?

    They need to be mut because later in the function they are mutated via sets[idx].clear() ...


    Value Expressions vs. Place Expressions
    - Place expressions represent a memory location. They include local variables, static variables, dereferences, array indexing expressions, field references, and parentheiszed place expressions.
    - Any other expression is a value expression. It represents an actual value.
    */
    let sets = [
        &mut HashSet::<char>::new(),
        &mut HashSet::<char>::new(),
        &mut HashSet::<char>::new(),
    ];

    for line_result in reader.lines() {
        let line = line_result?;
        sets[line_num % 3].clear();
        sets[line_num % 3].extend(line.chars());

        let is_last_line = line_num % 3 == 2;
        if is_last_line {
            /*
            What the heck is this doing?

            */
            let intersection = sets[0]
                .intersection(sets[1])
                .filter(|k| sets[2].contains(k))
                .collect::<Vec<&char>>()[0];
            if intersection.is_ascii_lowercase() {
                /*
                What is the * operator?
                * is a dereference operator, it gets the value inside the reference.
                `intersection` is a reference to a char (&char) so it needs to be dereferenced to get the char value.
                */
                score += *intersection as u32 - 96;
            } else if intersection.is_ascii_uppercase() {
                score += *intersection as u32 - 38;
            }
        }
        line_num += 1;
    }
    println!("Priority score is {:?}", score);
    Ok(())
}
