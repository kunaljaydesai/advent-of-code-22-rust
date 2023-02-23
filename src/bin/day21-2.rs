use std::{collections::HashMap, io::BufRead};

use regex::Regex;

#[derive(Clone)]
enum Dependency {
    Value(i64),
    Operation(String, String, String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day21.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let re = Regex::new(r"([a-z]+): ([0-9]+|(([a-z]+) (\+|\*|/|-) ([a-z]+)))").unwrap();
    let mut orig_dependencies = HashMap::<String, Dependency>::new();
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let cap = re.captures_iter(&line).next().unwrap();
        let monkey = String::from(&cap[1]);
        if let Some(val) = cap[2].parse::<i64>().ok() {
            orig_dependencies.insert(monkey, Dependency::Value(val));
        } else {
            let first_dependency = String::from(&cap[4]);
            let mut op = String::from(&cap[5]);
            if monkey == "root" {
                op = String::from("=");
            }
            let second_dependency = String::from(&cap[6]);
            orig_dependencies.insert(
                monkey,
                Dependency::Operation(first_dependency, op, second_dependency),
            );
        }
    }
    let mut humn_high = 3093175982596;
    let mut humn_low = 0;
    let mut humn = 0;
    loop {
        let mut dependencies = HashMap::new();
        dependencies.clone_from(&orig_dependencies);
        dependencies.insert(String::from("humn"), Dependency::Value(humn));
        'outer: loop {
            let mut overrides = HashMap::<String, i64>::new();
            for monkey in &dependencies {
                match monkey.1 {
                    Dependency::Operation(first, op, second) => {
                        let first = dependencies.get(first).unwrap();
                        let second = dependencies.get(second).unwrap();
                        let mut result = None;
                        match first {
                            Dependency::Value(first_val) => match second {
                                Dependency::Value(second_val) => match op.as_ref() {
                                    "+" => {
                                        result = Some(*first_val + *second_val);
                                    }
                                    "-" => {
                                        result = Some(*first_val - *second_val);
                                    }
                                    "*" => {
                                        result = Some(*first_val * *second_val);
                                    }
                                    "/" => {
                                        result = Some(*first_val / *second_val);
                                    }
                                    "=" => {
                                        if *first_val == *second_val {
                                            println!("humn val is {}", humn);
                                            return Ok(());
                                        } else {
                                            if *first_val > *second_val {
                                                humn_low = humn;
                                            }
                                            if *first_val < *second_val {
                                                humn_high = humn;
                                            }
                                            humn = (humn_low + humn_high) / 2;
                                            break 'outer;
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }

                        if let Some(val) = result {
                            overrides.insert(String::from(monkey.0), val);
                        }
                    }
                    _ => {}
                }
            }
            for val in &overrides {
                dependencies.insert(String::from(val.0), Dependency::Value(*val.1));
            }
        }
        humn += 1;
    }
    Ok(())
}
