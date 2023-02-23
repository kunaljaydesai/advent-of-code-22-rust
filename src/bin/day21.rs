use std::{collections::HashMap, io::BufRead};

use regex::Regex;

enum Dependency {
    Value(i64),
    Operation(String, String, String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day21.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let re = Regex::new(r"([a-z]+): ([0-9]+|(([a-z]+) (\+|\*|/|-) ([a-z]+)))").unwrap();
    let mut dependencies = HashMap::<String, Dependency>::new();
    for line_result in lines {
        let line = line_result.ok().unwrap();
        let cap = re.captures_iter(&line).next().unwrap();
        let monkey = String::from(&cap[1]);
        if let Some(val) = cap[2].parse::<i64>().ok() {
            dependencies.insert(monkey, Dependency::Value(val));
        } else {
            let first_dependency = String::from(&cap[4]);
            let op = String::from(&cap[5]);
            let second_dependency = String::from(&cap[6]);
            dependencies.insert(
                monkey,
                Dependency::Operation(first_dependency, op, second_dependency),
            );
        }
    }
    loop {
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
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    }

                    if let Some(val) = result {
                        if monkey.0 == "root" {
                            println!("Root: {}", val);
                            return Ok(());
                        }
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
    Ok(())
}
