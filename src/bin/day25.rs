use std::io::BufRead;

fn snafu_to_decimal(c: char) -> i64 {
    let decimal = match c {
        '1' => 1,
        '2' => 2,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => {
            panic!("Invalid char");
        }
    };
    decimal
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day25.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut sum_of_snafu = 0;
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let chars = line.chars().rev();
        let mut snafu: i64 = 0;
        let mut idx = 0;
        for char in chars {
            let decimal = snafu_to_decimal(char);
            snafu = snafu.checked_add(decimal * (5_i64.pow(idx))).unwrap();
            idx += 1;
        }
        sum_of_snafu += snafu;
    }

    let mut snafu = String::new();
    while sum_of_snafu > 0 {
        let remainder = sum_of_snafu.rem_euclid(5);
        sum_of_snafu = (sum_of_snafu + 2) / 5;
        let digit = match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => {
                panic!("Error: invalid input");
            }
        };
        snafu.push(digit);
    }
    let snafu_str = snafu.chars().rev().collect::<String>();
    println!("snafu {}", snafu_str);
    Ok(())
}
