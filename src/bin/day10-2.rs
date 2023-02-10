use core::num;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day10.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut num_cycles: usize = 0;
    let mut register: isize = 1;
    let mut pixels = [['.'; 40]; 6];

    for line_result in reader.lines() {
        let line = line_result?;
        let instruction = line.split(" ").nth(0).unwrap();

        num_cycles += 1;
        if register >= 0
            && (((num_cycles - 1) % 40) == register as usize
                || ((num_cycles - 1) % 40) == (register - 1) as usize
                || ((num_cycles - 1) % 40) == (register + 1) as usize)
        {
            let location = num_cycles - 1;
            pixels[location / 40][location % 40] = '#';
        }

        if instruction == "addx" {
            num_cycles += 1;
            if register >= 0
                && (((num_cycles - 1) % 40) == register as usize
                    || ((num_cycles - 1) % 40) == (register - 1) as usize
                    || ((num_cycles - 1) % 40) == (register + 1) as usize)
            {
                let location = num_cycles - 1;
                pixels[location / 40][location % 40] = '#';
            }
            let operand = line.split(" ").nth(1).unwrap().parse::<isize>().unwrap();
            register += operand;
        }
    }
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        pixels[0], pixels[1], pixels[2], pixels[3], pixels[4], pixels[5]
    );
    Ok(())
}
