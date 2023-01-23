use std::io::BufRead;

struct ParsedLine {
    count: u32,
    from: u32,
    to: u32,
}

pub fn get_number(line: &str) -> u32 {
    let mut num = 0;
    let str_vec: Vec<char> = line.chars().collect();
    for c in str_vec {
        if !c.is_digit(10) {
            return num;
        }
        num = num * 10 + c.to_digit(10).unwrap();
    }
    return num;
}

impl ParsedLine {
    pub fn from_str(line: &String) -> ParsedLine {
        let count = get_number(&line[5..]);
        let from = get_number(&line[line.find("from ").unwrap() + 5..]);
        let to = get_number(&line[line.find("to ").unwrap() + 3..]);
        return ParsedLine {
            count: count,
            from: from - 1,
            to: to - 1,
        };
    }
}

fn print_crate(crates: &[Vec<char>; 9]) {
    for v in crates {
        if v.len() == 0 {
            print!(" ");
        } else {
            print!("{}", v.last().unwrap());
        }
    }
    println!("");
}

fn main() -> Result<(), std::io::Error> {
    let mut crates: [Vec<char>; 9] = [
        vec!['Z', 'P', 'M', 'H', 'R'],
        vec!['P', 'C', 'J', 'B'],
        vec!['S', 'N', 'H', 'G', 'L', 'C', 'D'],
        vec!['F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L'],
        vec!['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M'],
        vec!['T', 'F', 'S', 'Z', 'B', 'G'],
        vec!['N', 'R', 'V'],
        vec!['P', 'G', 'L', 'T', 'D', 'V', 'C', 'M'],
        vec!['W', 'Q', 'N', 'J', 'F', 'M', 'L'],
    ];
    let path = std::path::Path::new("day5.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let parsed_line = ParsedLine::from_str(&line);
        for _ in 0..parsed_line.count {
            let item = crates[parsed_line.from as usize].pop();
            if item.is_some() {
                crates[parsed_line.to as usize].push(item.unwrap());
            }
        }
    }
    print_crate(&crates);
    Ok(())
}
