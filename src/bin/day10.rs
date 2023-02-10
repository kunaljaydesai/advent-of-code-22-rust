use core::num;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day10.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut num_cycles = 0;
    let mut register = 1;
    let mut signal_strength_sum = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let instruction = line.split(" ").nth(0).unwrap();
        if instruction == "noop" {
            num_cycles += 1;
            if num_cycles == 20
                || num_cycles == 60
                || num_cycles == 100
                || num_cycles == 140
                || num_cycles == 180
                || num_cycles == 220
            {
                let signal_strength = num_cycles * register;
                signal_strength_sum += signal_strength;
            }
        } else if instruction == "addx" {
            let operand = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            num_cycles += 1;
            if num_cycles == 20
                || num_cycles == 60
                || num_cycles == 100
                || num_cycles == 140
                || num_cycles == 180
                || num_cycles == 220
            {
                let signal_strength = num_cycles * register;
                signal_strength_sum += signal_strength;
            }
            num_cycles += 1;
            if num_cycles == 20
                || num_cycles == 60
                || num_cycles == 100
                || num_cycles == 140
                || num_cycles == 180
                || num_cycles == 220
            {
                let signal_strength = num_cycles * register;
                signal_strength_sum += signal_strength;
            }
            register += operand;
        }
    }
    println!("signal strength sum: {}", signal_strength_sum);
    Ok(())
}
